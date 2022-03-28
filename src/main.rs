pub mod home;
pub mod session;
pub mod user;
pub mod util;

use crate::{
    home::{controller::index_tree, view::home_page},
    session::view::login_page,
    util::get_position,
};
use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Handled, Lens, Selector, Target,
    WindowDesc, WindowId,
};

const NEW_HOME_WINDOW: Selector<i32> = Selector::new("new-home-window");

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct AppModel {
    pub session: session::ViewModel,
    pub home: home::ViewModel,
}

struct Delegate {
    windows: Vec<WindowId>,
}

impl AppDelegate<AppModel> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppModel,
        _env: &Env,
    ) -> Handled {
        match cmd {
            _ if cmd.is(NEW_HOME_WINDOW) => {
                let user_id = cmd.get_unchecked(NEW_HOME_WINDOW);
                index_tree(&mut data.home, *user_id);
                let new_win = WindowDesc::new(home_page).window_size((800.0, 600.0));
                ctx.new_window(new_win);
                Handled::Yes
            }
            _ => Handled::No,
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _data: &mut AppModel,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window added, id: {:?}", id);
        self.windows.push(id);
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        _data: &mut AppModel,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window removed, id: {:?}", id);
        if let Some(pos) = self.windows.iter().position(|x| *x == id) {
            self.windows.remove(pos);
        }
    }
}

fn main() {
    let width = 360.0;
    let height = 200.0;
    let window = WindowDesc::new(login_page)
        .title("hello window")
        .window_size((width, height))
        .resizable(false)
        .set_position(get_position(width, height));
    let state = AppModel {
        session: session::ViewModel::default(),
        home: home::ViewModel::default(),
    };
    AppLauncher::with_window(window)
        .delegate(Delegate {
            windows: Vec::new(),
        })
        .launch(state)
        .unwrap();
}
