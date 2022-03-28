pub mod controller;
pub mod view;

use druid::{Data, Lens};

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct ViewModel {
    pub user_name: String,
    pub password: String,
}
