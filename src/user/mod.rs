pub mod controller;
pub mod view;

use druid::{im::Vector, Data, Lens};

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct Model {
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct ViewModel {
    pub user_name: String,
    pub password: String,
    pub users: Vector<Model>,
}
