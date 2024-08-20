use anyhow::anyhow;
use serde::{Deserialize, Serialize};
//use slint::ComponentHandle;

const PLACEHOLDER_TASKS: [&str; 8] = [
    "Research topic and gather resources",
    "Draft initial outline",
    "Complete first draft",
    "Review and edit content",
    "Design visual elements",
    "Prepare final presentation",
    "Conduct final review",
    "Submit completed work",
];

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub complete: bool,
    pub description: String,
}

impl Task {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            complete: false,
            description: PLACEHOLDER_TASKS[rand::random::<usize>() % 8].to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct List {
    pub id: usize,
    pub name: String,
    pub tasks: Vec<Task>,
}

impl List {
    pub fn new(id: usize, name: String) -> Self {
        List {
            id: id,
            name: name,
            tasks: vec![],
        }
    }

    pub fn add_task(&mut self) {
        let index = self.tasks.len();

        self.tasks.insert(index, Task::new(index));
    }

    pub fn update_task(&mut self, task: Task) -> anyhow::Result<()> {
        if task.id >= self.tasks.len() {
            return Err(anyhow!("Id out of range"));
        }

        self.tasks[task.id] = task.clone();

        Ok(())
    }

    pub fn remove_task(&mut self, task_id: usize) -> anyhow::Result<()> {
        if task_id >= self.tasks.len() {
            return Err(anyhow!("Id out of range"));
        }

        self.tasks.remove(task_id);

        for i in task_id..self.tasks.len() {
            self.tasks[i].id -= 1;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ListReduced {
    pub id: usize,
    pub name: String,
}

impl std::convert::From<List> for ListReduced {
    fn from(value: List) -> Self {
        ListReduced {
            id: value.id,
            name: value.name,
        }
    }
}

pub trait IoHandlerTrait {
    fn set_active_list(&self, list_id: usize) -> anyhow::Result<List>;

    fn add_task(&self);

    fn update_task(&self, task: Task) -> anyhow::Result<()>;

    fn update_list(&self, name: String);

    fn add_list(&self);

    fn remove_list(&self, list_id: usize) -> anyhow::Result<()>;

    fn remove_task(&self, task_id: usize) -> anyhow::Result<()>;

    fn get_all_reduced_lists(&self) -> Vec<ListReduced>;

    fn get_active_list(&self) -> List;
}
