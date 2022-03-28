use crate::{
    home::{self, controller::expand_tree},
    AppModel,
};
use druid::{
    widget::{
        Axis, Container, CrossAxisAlignment, Flex, Label, Split, Tabs, TabsEdge, ViewSwitcher,
    },
    Widget, WidgetExt,
};

pub fn home_page() -> impl Widget<AppModel> {
    let left_page = ViewSwitcher::new(
        |am: &AppModel, _| am.home.clone(),
        |vm: &home::ViewModel, _, _| Box::new(tree_page(vm).lens(AppModel::home)),
    );
    let right_page = Tabs::for_policy(home::NumberedTabs)
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_transition(Default::default())
        .lens(AppModel::home);
    Split::columns(left_page, right_page)
        .split_point(0.3)
        .solid_bar(true)
        .bar_size(1.0)
        .draggable(true)
}

fn tree_page(vm: &home::ViewModel) -> impl Widget<home::ViewModel> {
    let col = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_default_spacer();
    vm.trees.iter().enumerate().fold(col, |c, (index, tree)| {
        let item = tree.clone();
        let lv = item.level;
        c.with_child(
            Container::new(Label::new(item.name.clone()).on_click(
                move |_ctx, data: &mut home::ViewModel, _env| {
                    expand_tree(data, index);
                },
            ))
            .padding(((20 * lv) as f64, 0.0, 0.0, 0.0)),
        )
    })
}
