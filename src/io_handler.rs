use std::{cell::RefCell, fs};

use anyhow::anyhow;

use crate::io_handler_trait::{IoHandlerTrait, List, ListReduced, Task};

#[derive(Clone, Debug)]
pub struct IoHandlerFs {
    file_path: String,
    active_list: RefCell<usize>,
    lists: RefCell<Vec<List>>,
}

impl IoHandlerFs {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            active_list: RefCell::new(0),
            lists: RefCell::new(vec![List::new(0, "Untitled list 1".to_string())]),
        }
    }

    pub fn write_to_file(&self) -> anyhow::Result<()> {
        fs::write(&self.file_path, serde_json::to_string(&self.lists)?)?;

        Ok(())
    }

    pub fn read_from_file(&self) -> anyhow::Result<()> {
        let contents = fs::read_to_string(&self.file_path)?;
        let mut lists_ref = self.lists.borrow_mut();

        *lists_ref = serde_json::from_str::<Vec<List>>(&contents)?;

        Ok(())
    }
}

impl IoHandlerTrait for IoHandlerFs {
    fn set_active_list(&self, list_id: usize) -> anyhow::Result<List> {
        if list_id >= self.lists.borrow().len() {
            return Err(anyhow!("Id out of range of possible ids"));
        }

        let mut ind_ref = self.active_list.borrow_mut();
        let return_list = self.lists.borrow();

        *ind_ref = list_id;

        Ok(return_list[list_id].clone())
    }

    fn add_task(&self) {
        let mut lists_ref = self.lists.borrow_mut();
        lists_ref[*self.active_list.borrow()].add_task()
    }

    fn update_task(&self, task: Task) -> anyhow::Result<()> {
        let mut lists_ref = self.lists.borrow_mut();
        lists_ref[*self.active_list.borrow()].update_task(task)
    }

    fn update_list(&self, name: String) {
        let mut lists_ref = self.lists.borrow_mut();
        lists_ref[*self.active_list.borrow()].name = name
    }

    fn add_list(&self) {
        let index = self.lists.borrow().len();
        let mut lists_ref = self.lists.borrow_mut();

        lists_ref.insert(index, List::new(index, format!("Untitled list {}", index)));
    }

    fn remove_list(&self, list_id: usize) -> anyhow::Result<()> {
        if list_id >= self.lists.borrow().len() {
            return Err(anyhow!("Id out of range of possible ids"));
        }

        let mut lists_ref = self.lists.borrow_mut();

        let _ = lists_ref.remove(list_id);

        for i in list_id - 1..self.lists.borrow().len() {
            lists_ref[i].id -= 1;
        }

        Ok(())
    }

    fn remove_task(&self, task_id: usize) -> anyhow::Result<()> {
        let mut lists_ref = self.lists.borrow_mut();

        lists_ref[self.active_list.borrow().clone()].remove_task(task_id)
    }

    fn get_all_reduced_lists(&self) -> Vec<ListReduced> {
        self.lists
            .borrow()
            .clone()
            .into_iter()
            .map(ListReduced::from)
            .collect()
    }

    fn get_active_list(&self) -> List {
        let lists_ref = self.lists.borrow();

        lists_ref[self.active_list.borrow().clone()].clone()
    }
}
