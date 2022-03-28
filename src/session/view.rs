use crate::{
    session::{
        self,
        controller::{sign_in, sign_up},
    },
    AppModel,
};
use druid::{
    widget::{Button, Flex, TextBox},
    KeyOrValue, Widget, WidgetExt,
};

pub fn login_page() -> impl Widget<AppModel> {
    let bts = Flex::row()
        .with_child(
            Button::new("注 册")
                .fix_width(80.0)
                .on_click(|_ctx, vm: &mut session::ViewModel, _env| sign_up(vm)),
        )
        .with_spacer(KeyOrValue::Concrete(20.0))
        .with_child(
            Button::new("登 录")
                .fix_width(80.0)
                .on_click(|ctx, vm, _env| sign_in(ctx, vm)),
        )
        .lens(AppModel::session);
    let ips = Flex::column()
        .with_child(
            TextBox::new()
                .with_placeholder("请输入用户名")
                .fix_width(180.0)
                .lens(session::ViewModel::user_name),
        )
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("请输入密码")
                .fix_width(180.0)
                .lens(session::ViewModel::password),
        )
        .lens(AppModel::session);
    Flex::column()
        .with_child(ips)
        .with_default_spacer()
        .with_child(bts)
        .padding((0.0, 38.0, 0.0, 0.0))
}
