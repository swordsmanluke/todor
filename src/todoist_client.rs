use restson::{RestPath, Error, RestClient};
use chrono::{DateTime, Local, SecondsFormat, Utc, TimeZone};
use log::info;

const URL_BASE: &str = "https://api.todoist.com/";

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Project {
    comment_count: u64,
    id: u64,
    name: String,
    color: u64,
    shared: bool
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Task {
    pub id: u64,
    pub project_id: u64,
    pub section_id: u64,
    pub content: String,
    pub completed: bool,
    pub label_ids: Vec<u64>,
    pub parent_id: Option<u64>,
    pub order: Option<u64>,
    pub priority: u64,
    pub due_string: Option<String>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due: Option<TodoistDate>,
    pub url: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct NewTask {
    pub project_id: u64,
    pub content: String,
    pub due_datetime: Option<String>,
}

#[derive(Serialize)]
struct TaskClose{}

impl Task {
    pub fn new(pid: u64, id: Option<u64>, description: String, due: DateTime<Local>) -> Task {
        Task {
            id: id.unwrap_or(0),
            project_id: pid,
            section_id: 0,
            content: description,
            completed: false,
            label_ids: vec![],
            parent_id: None,
            order: None,
            priority: 0,
            due_string: None,
            due_date: None,
            due_datetime: Some(due.to_rfc3339()),
            due: None,
            url: "".to_string()
        }
    }
}

impl NewTask {
    pub fn new(pid: u64, description: String, due: DateTime<Local>) -> NewTask {
        NewTask {
            project_id: pid,
            content: description,
            due_datetime: Some(Utc.from_local_datetime(&due.naive_local()).unwrap().to_rfc3339_opts(SecondsFormat::Secs, true)),
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TodoistDate {
    pub string: String,
    pub date: String,  // Date in format YYYY-MM-DD corrected to user’s timezone.
    pub datetime: Option<String>, // date and time in RFC3339 format in UTC.
    pub timezone: Option<String>, // Only returned if exact due time set, user’s timezone
                                  // definition either in tzdata-compatible format (“Europe/Berlin”)
                                  // or as a string specifying east of UTC offset as “UTC±HH:MM”
                                  // (i.e. “UTC-01:00”).
}

pub trait TodoistClient {
    fn projects(&self) -> Result<Vec<Project>, Error>;
    fn tasks(&self, project: &str) -> Result<Vec<Task>, Error>;
    fn add(&self, project: &str, task: String, due_date: Option<DateTime<Local>>) -> Result<bool, Error>;
    fn reschedule(&self, project: &str, task_id: u64, content: String, due_date: Option<DateTime<Local>>) -> Result<bool, Error>;
    fn close(&self, task_id: u64) ->  Result<bool, Error>;
}

pub struct TodoistRestClient {
    token: String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
struct Projects ( pub Vec<Project> ); // alias to help deserialization

#[derive(Serialize,Deserialize,Debug,Clone)]
struct Tasks ( pub Vec<Task> ); // alias to help deserialization

// get("https://api.todoist.com/rest/v1/projects", headers={"Authorization": "Bearer %s" % your_token}).json()

impl RestPath<()> for Projects {
    fn get_path(_: ()) -> Result<String,Error> { Ok("rest/v1/projects".to_string()) }
}

impl RestPath<()> for Tasks {
    fn get_path(_: ()) -> Result<String,Error> { Ok("rest/v1/tasks".to_string()) }
}

impl RestPath<()> for Task {
    fn get_path(_: ()) -> Result<String,Error> { Ok("rest/v1/tasks".to_string()) }
}

impl RestPath<()> for NewTask {
    fn get_path(_: ()) -> Result<String,Error> { Ok("rest/v1/tasks".to_string()) }
}

impl RestPath<u64> for Task {
    fn get_path(task_id: u64) -> Result<String,Error> {
        Ok(format!("rest/v1/tasks/{}", task_id))
    }
}

impl RestPath<u64> for TaskClose {
    fn get_path(task_id: u64) -> Result<String,Error> { Ok(format!("rest/v1/tasks/{}/close", task_id)) }
}

impl TodoistRestClient {
    pub fn new(token: String) -> TodoistRestClient {
        TodoistRestClient { token }
    }

    fn get_client(&self) -> Result<RestClient, Error> {
        let mut client = RestClient::new(URL_BASE)?;
        client.set_header("Authorization", format!("Bearer {}", self.token).as_str())?;
        Ok(client)
    }
}
impl TodoistClient for TodoistRestClient {
    fn projects(&self) -> Result<Vec<Project>, Error> {
        let mut client = self.get_client()?;
        let projects = client.get::<_, Projects>(())?.0;

        Ok(projects)
    }

    fn tasks(&self, project: &str) -> Result<Vec<Task>, Error> {
        let mut client = self.get_client()?;
        let projects = self.projects()?;
        let selected_project = projects.iter().find(|p| p.name == project).
            expect(format!("No project named {}", project).as_str());

        let tasks: Vec<Task> = client.get_with::<_, Tasks>((), &[("project_id", format!("{}", selected_project.id).as_str())])?.0.iter().
            map(|t| t.to_owned()).
            collect();
        Ok(tasks)
    }

    fn add(&self, project: &str, task: String, due_date: Option<DateTime<Local>>) -> Result<bool, Error> {
        let mut client = self.get_client()?;
        let projects = self.projects()?;
        let selected_project = projects.iter().find(|p| p.name == project).
            expect(format!("No project named {}", project).as_str());

        let data = NewTask::new(selected_project.id, task, due_date.unwrap_or(Local::now()));
        info!("Creating Todoist Task: {:?}", data);
        client.post((), &data)?;

        Ok(true)
    }

    fn reschedule(&self, project: &str, task_id: u64, content: String, due_date: Option<DateTime<Local>>) -> Result<bool, Error> {
        let mut client = self.get_client()?;
        let projects = self.projects()?;
        let selected_project = projects.iter().find(|p| p.name == project).
            expect(format!("No project named {}", project).as_str());

        let data = Task::new(selected_project.id, Some(task_id), content, due_date.unwrap_or(Local::now()));
        info!("Rescheduling Todoist Task: {:?}", data);
        client.post(task_id, &data)?;

        Ok(true)
    }

    fn close(&self, task_id: u64) -> Result<bool, Error> {
        let mut client = self.get_client()?;
        let task_close = TaskClose{};
        client.post(task_id, &task_close)?;

        Ok(true)
    }
}

