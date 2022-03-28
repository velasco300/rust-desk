use crate::{
    home,
    user::{
        self,
        controller::{add_user, delete_user, index_user},
    },
};
use druid::{
    widget::{Button, Flex, Label, TextBox, ViewSwitcher},
    Widget, WidgetExt,
};

pub fn user_page() -> impl Widget<home::ViewModel> {
    ViewSwitcher::new(
        |hvm: &home::ViewModel, _| hvm.user.clone(),
        |uvm: &user::ViewModel, _, _| Box::new(index_page(uvm).lens(home::ViewModel::user)),
    )
}

fn index_page(vm: &user::ViewModel) -> impl Widget<user::ViewModel> {
    let mut col = Flex::column().with_child(
        Flex::row()
            .with_child(
                TextBox::new()
                    .with_placeholder("输入用户名")
                    .lens(user::ViewModel::user_name),
            )
            .with_child(
                TextBox::new()
                    .with_placeholder("输入密码")
                    .lens(user::ViewModel::password),
            )
            .with_child(
                Button::new("新 增").on_click(|_ctx, vm: &mut user::ViewModel, _env| add_user(vm)),
            )
            .with_child(
                Button::new("查 询")
                    .on_click(|_ctx, vm: &mut user::ViewModel, _env| index_user(vm)),
            ),
    );

    for item in vm.users.iter().enumerate() {
        let idx = item.0;
        let u = item.1;
        col = col.with_child(
            Flex::row()
                .with_child(Label::new(u.user_name.clone()))
                .with_child(Label::new(u.password.clone()))
                .with_child(
                    Button::new("删 除")
                        .on_click(move |_ctx, vm: &mut user::ViewModel, _env| delete_user(vm, idx)),
                ),
        );
    }
    col
}
