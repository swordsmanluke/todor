use restson::{RestPath, Error, RestClient};
use chrono::{DateTime, Local};

const URL_BASE: &str = "https://api.todoist.com/";

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Project {
    comment_count: usize,
    id: usize,
    name: String,
    color: usize,
    shared: bool
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Task {
    pub id: usize,
    pub project_id: usize,
    pub section_id: usize,
    pub content: String,
    pub completed: bool,
    pub label_ids: Vec<usize>,
    pub parent: Option<usize>,
    pub order: Option<usize>,
    pub priority: usize,
    pub due: Option<TodoistDate>,
    pub url: String
}

#[derive(Serialize)]
struct TaskClose{}

impl Task {
    pub fn from(pid: usize, s: String, due: DateTime<Local>) -> Task {
        Task {
            id: 0,
            project_id: pid,
            section_id: 0,
            content: s,
            completed: false,
            label_ids: vec![],
            parent: None,
            order: None,
            priority: 0,
            due: Some(TodoistDate{
                string: due.to_rfc3339(),
                date: due.date().to_string(),
                datetime: None,
                timezone: None
            }),
            url: "".to_string()
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
    fn add(&self, project: &str, task: String) -> Result<bool, Error>;
    fn close(&self, task_id: usize) ->  Result<bool, Error>;
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

impl RestPath<usize> for TaskClose {
    fn get_path(task_id: usize) -> Result<String,Error> { Ok(format!("rest/v1/tasks/{}/close", task_id)) }
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

    fn add(&self, project: &str, task: String) -> Result<bool, Error> {
        let mut client = self.get_client()?;
        let projects = self.projects()?;
        let selected_project = projects.iter().find(|p| p.name == project).
            expect(format!("No project named {}", project).as_str());

        let data = Task::from(selected_project.id, task, Local::now());
        client.post((), &data)?;

        Ok(true)
    }

    fn close(&self, task_id: usize) -> Result<bool, Error> {
        let mut client = self.get_client()?;
        let task_close = TaskClose{};
        println!("url: {}", TaskClose::get_path(task_id)?);
        client.post(task_id, &task_close)?;


        Ok(true)
    }
}

