use restson::{RestPath, Error, RestClient};

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
    id: usize,
    project_id: usize,
    section_id: usize,
    content: String,
    completed: bool,
    label_ids: Vec<usize>,
    parent: Option<usize>,
    order: Option<usize>,
    priority: usize,
    due: Option<TodoistDate>,
    url: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TodoistDate {
    string: String,
    date: String,  // Date in format YYYY-MM-DD corrected to user’s timezone.
    datetime: Option<String>, // date and time in RFC3339 format in UTC.
    timezone: Option<String>, // Only returned if exact due time set, user’s timezone
                              // definition either in tzdata-compatible format (“Europe/Berlin”)
                              // or as a string specifying east of UTC offset as “UTC±HH:MM”
                              // (i.e. “UTC-01:00”).
}

pub struct TodoistClient {
    token: String,
    projects: Vec<Project>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
struct Projects ( pub Vec<Project> ); // alias to help deserialization

#[derive(Serialize,Deserialize,Debug,Clone)]
struct Tasks ( pub Vec<Task> ); // alias to help deserialization

// get("https://api.todoist.com/rest/v1/projects", headers={"Authorization": "Bearer %s" % your_token}).json()

impl RestPath<()> for Projects {
    fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("rest/v1/projects")) }
}

impl RestPath<()> for Tasks {
    fn get_path(_: ()) -> Result<String,Error> { Ok(format!("rest/v1/tasks")) }
}

impl TodoistClient {
    pub fn new(token: String) -> TodoistClient {
        TodoistClient { token, projects: Vec::new() }
    }

    pub fn projects(&mut self) -> &Vec<Project> {
        if self.projects.is_empty() {
            let mut client = self.get_client();
            self.projects = match client.get::<_, Projects>(()) {
                Ok(projects) => projects.0,
                Err(_) => Vec::new()
            };
        }

        &self.projects
    }

    pub fn tasks(&mut self, project: &str) -> Vec<Task> {
        let mut client = self.get_client();
        let projects = self.projects();
        let selected_project = projects.iter().find(|p| p.name == project).expect(format!("No project named {}", project).as_str());
        println!("Selected Project: {} -> {:?}", project, selected_project);

        let tasks: Vec<Task> = client.get::<_, Tasks>(()).unwrap().0.iter().
            filter(|t| t.project_id == selected_project.id).
            map(|t| t.to_owned()).
            collect();
        tasks
    }

    fn get_client(&mut self) -> RestClient {
        let mut client = RestClient::new(URL_BASE).unwrap();
        client.set_header("Authorization", format!("Bearer {}", self.token).as_str());
        client
    }
}

