use crate::{
    home,
    user::{
        self,
        controller::{add_user, delete_user, index_user},
    },
};
use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox, ViewSwitcher},
    KeyOrValue, Widget, WidgetExt,
};

pub fn user_page() -> impl Widget<home::ViewModel> {
    ViewSwitcher::new(
        |hvm: &home::ViewModel, _| hvm.user.clone(),
        |uvm: &user::ViewModel, _, _| Box::new(index_page(uvm).lens(home::ViewModel::user)),
    )
}

fn index_page(vm: &user::ViewModel) -> impl Widget<user::ViewModel> {
    let mut col = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Flex::row()
                .with_child(
                    TextBox::new()
                        .with_placeholder("输入用户名")
                        .lens(user::ViewModel::user_name),
                )
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_placeholder("输入密码")
                        .lens(user::ViewModel::password),
                )
                .with_flex_spacer(1.0)
                .with_child(
                    Button::new("新 增")
                        .on_click(|_ctx, vm: &mut user::ViewModel, _env| add_user(vm)),
                )
                .with_default_spacer()
                .with_child(
                    Button::new("查 询")
                        .on_click(|_ctx, vm: &mut user::ViewModel, _env| index_user(vm)),
                ),
        )
        .with_spacer(KeyOrValue::Concrete(10.0));

    for item in vm.users.iter().enumerate() {
        let idx = item.0;
        let u = item.1;
        col = col.with_spacer(KeyOrValue::Concrete(6.0)).with_child(
            Flex::row()
                .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .with_child(Label::new(u.user_name.clone()))
                .with_child(Label::new(u.password.clone()))
                .with_child(
                    Button::new("删 除")
                        .on_click(move |_ctx, vm: &mut user::ViewModel, _env| delete_user(vm, idx)),
                )
                .expand_width(),
        );
    }
    col.padding((2.0, 10.0))
}
