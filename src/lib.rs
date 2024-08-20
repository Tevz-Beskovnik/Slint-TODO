use std::rc::Rc;

use slint::{ModelRc, VecModel};

pub mod io_handler;
pub mod io_handler_trait;

pub fn vec_to_model_rc<T: Clone + 'static>(input: Vec<T>) -> ModelRc<T> {
    ModelRc::from(Rc::new(VecModel::from(input)))
}
