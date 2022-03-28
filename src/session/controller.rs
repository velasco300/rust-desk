use crate::{session, NEW_HOME_WINDOW};
use druid::EventCtx;

pub fn sign_up(vm: &mut session::ViewModel) {
    vm.password = 123456.to_string();
}

pub fn sign_in(ctx: &mut EventCtx, vm: &mut session::ViewModel) {
    println!("use the user name and password query from db {:?}", vm);
    ctx.submit_command(NEW_HOME_WINDOW.with(123));
    ctx.window().close();
}
