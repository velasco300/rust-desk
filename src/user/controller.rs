use crate::user;

pub fn index_user(vm: &mut user::ViewModel) {
    vm.users.push_back(user::Model {
        user_name: "tom".to_string(),
        password: "123456".to_string(),
    });
}

pub fn add_user(vm: &mut user::ViewModel) {
    vm.users.push_back(user::Model {
        user_name: vm.user_name.clone(),
        password: vm.password.clone(),
    });
}

pub fn delete_user(vm: &mut user::ViewModel, idx: usize) {
    vm.users.remove(idx);
}
