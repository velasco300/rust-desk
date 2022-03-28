pub mod controller;
pub mod view;

use crate::user::{self, view::user_page};
use druid::{
    im::{vector, Vector},
    widget::{Container, Label, TabInfo, TabsPolicy},
    Data, Lens,
};

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct Tree {
    pub id: i32,
    pub pid: i32,
    pub root_id: i32,
    pub name: String,
    pub tab_type: TabType,
    pub url: String,
    pub is_leaf: bool,
    pub level: i32,
    pub children: Vector<Tree>,
    pub items: Vector<Tree>,
}
impl Tree {
    fn dir(root: &mut Tree, arr: &mut Vector<Tree>) {
        let item = Tree {
            id: root.id,
            pid: root.pid,
            root_id: root.root_id,
            name: root.name.clone(),
            tab_type: root.tab_type,
            url: root.url.clone(),
            is_leaf: root.is_leaf,
            level: root.level,
            children: vector![],
            items: vector![],
        };
        arr.push_back(item);
        for tree in root.children.iter_mut() {
            tree.level = root.level + 1;
            Self::dir(tree, arr)
        }
    }

    pub fn to_items(&mut self) {
        let mut arr: Vector<Tree> = vector![];
        Self::dir(self, &mut arr);
        self.items = arr;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Data)]
pub enum TabType {
    EmptyPage,
    UserPage,
}
impl Default for TabType {
    fn default() -> Self {
        Self::EmptyPage
    }
}

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct Tab {
    pub tab_type: TabType,
    pub title: String,
}

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct ViewModel {
    pub trees: Vector<Tree>,
    pub tabs: Vector<Tab>,
    pub user: user::ViewModel,
}
impl ViewModel {
    fn add_tab(&mut self, tab: Tab) {
        let mut addble = true;
        for t in &self.tabs {
            if t.title == tab.title {
                addble = false;
                break;
            }
        }
        if addble {
            self.tabs.push_back(tab);
        }
    }

    fn remove_tab(&mut self, idx: usize) {
        self.tabs.remove(idx);
    }

    fn tabs_key(&self) -> usize {
        self.tabs.len()
    }
}

#[derive(Clone, Data)]
pub struct NumberedTabs;
impl TabsPolicy for NumberedTabs {
    type Key = usize;
    type Build = ();
    type Input = ViewModel;
    type LabelWidget = Label<ViewModel>;
    type BodyWidget = Container<ViewModel>;

    fn tabs_changed(&self, old_data: &ViewModel, data: &ViewModel) -> bool {
        old_data.tabs_key() != data.tabs_key()
    }

    fn tabs(&self, data: &ViewModel) -> Vec<Self::Key> {
        data.tabs.iter().enumerate().map(|item| item.0).collect()
    }

    fn tab_info(&self, key: Self::Key, data: &ViewModel) -> TabInfo<ViewModel> {
        TabInfo::new(format!("{}", data.tabs.get(key).unwrap().title), true)
    }

    fn tab_body(&self, key: Self::Key, data: &ViewModel) -> Self::BodyWidget {
        let tab = data.tabs.get(key);
        if tab.is_some() {
            let t = tab.unwrap();
            match t.tab_type {
                TabType::EmptyPage => Container::new(Label::new("empty page")),
                TabType::UserPage => Container::new(user_page()),
            }
        } else {
            Container::new(Label::new("default page"))
        }
    }

    fn close_tab(&self, key: Self::Key, data: &mut ViewModel) {
        data.remove_tab(key)
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
}
