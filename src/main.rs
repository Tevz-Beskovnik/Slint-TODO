use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString};
use slint_test::io_handler::IoHandlerFs;
use slint_test::io_handler_trait::{IoHandlerTrait, Task as TaskInternal};
use slint_test::vec_to_model_rc;

slint::include_modules!();

pub fn task_list_to_model_rc(input: Vec<TaskInternal>) -> ModelRc<Task> {
    vec_to_model_rc(
        input
            .into_iter()
            .map(|x| Task {
                complete: x.complete,
                description: SharedString::from(&x.description),
                id: x.id as i32,
            })
            .collect::<Vec<Task>>(),
    )
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let io_handler_rc = Rc::new(IoHandlerFs::new("./todos.json".to_string()));
    let _ = io_handler_rc.read_from_file();

    let todo_logic = ui.global::<TodoLogic>();
    let list = io_handler_rc.get_active_list();
    let list_tasks = task_list_to_model_rc(list.tasks.clone());

    todo_logic.set_list_name(SharedString::from(&list.name));
    todo_logic.set_task_list(list_tasks);
    todo_logic.set_list_index(0);

    todo_logic.on_update_task({
        let io_handler_rc = io_handler_rc.clone();
        move |description, status, id| {
            let _ = io_handler_rc.update_task(TaskInternal {
                id: id.clone() as usize,
                complete: status.clone(),
                description: description.clone().to_string(),
            });
            let _ = io_handler_rc.write_to_file();
        }
    });

    todo_logic.on_add_task({
        let weak_update = ui.as_weak();
        let io_handler_rc = io_handler_rc.clone();
        move || {
            let weak_app = weak_update.unwrap();
            let weak_todo_logic = weak_app.global::<TodoLogic>();

            let _ = io_handler_rc.add_task();
            let list = io_handler_rc.get_active_list();
            weak_todo_logic.set_task_list(task_list_to_model_rc(list.tasks));

            let _ = io_handler_rc.write_to_file();
        }
    });

    todo_logic.on_update_list_name({
        let io_handler_rc = io_handler_rc.clone();
        move |name| {
            let _ = io_handler_rc.update_list(name.to_string());
            let _ = io_handler_rc.write_to_file();
        }
    });

    todo_logic.on_update_active_index({
        let weak_update = ui.as_weak();
        let io_handler_rc = io_handler_rc.clone();
        move |list_id| {
            let weak_app = weak_update.unwrap();
            let weak_todo_logic = weak_app.global::<TodoLogic>();

            let _ = io_handler_rc.set_active_list(list_id as usize);
            let list = io_handler_rc.get_active_list();
            weak_todo_logic.set_list_name(SharedString::from(list.name.clone()));
            weak_todo_logic.set_task_list(task_list_to_model_rc(list.tasks.clone()));

            let _ = io_handler_rc.write_to_file();
        }
    });

    todo_logic.on_remove_task({
        let weak_update = ui.as_weak();
        let io_handler_rc = io_handler_rc.clone();
        move |task_id| {
            let weak_app = weak_update.unwrap();
            let weak_todo_logic = weak_app.global::<TodoLogic>();
            let _ = io_handler_rc.remove_task(task_id as usize);
            let list = io_handler_rc.get_active_list();
            weak_todo_logic.set_task_list(task_list_to_model_rc(list.tasks.clone()));

            let _ = io_handler_rc.write_to_file();
        }
    });

    ui.run()
}
