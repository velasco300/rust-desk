use crate::home::{self, Tab, TabType, Tree};

pub fn index_tree(vm: &mut home::ViewModel, user_id: i32) {
    println!("get info from db use user id {}", user_id);
    let mut root = Tree {
        id: 1,
        is_leaf: false,
        name: format!("tree{}", 1),
        ..Default::default()
    };
    for i in 2..5 {
        let mut t = Tree {
            id: i,
            is_leaf: true,
            name: format!("tree{}", i),
            ..Default::default()
        };
        for x in 1..5 {
            t.children.push_back(Tree {
                is_leaf: true,
                name: format!("user{}{}", i, x),
                tab_type: TabType::UserPage,
                ..Default::default()
            });
        }
        root.children.push_back(t);
    }
    root.to_items();
    vm.trees = root.items;
}

pub fn expand_tree(vm: &mut home::ViewModel, index: usize) {
    let tree = vm.trees.get(index).unwrap();
    if tree.is_leaf {
        let tab = Tab {
            tab_type: tree.tab_type,
            title: tree.name.clone(),
        };
        vm.add_tab(tab);
    } else {
        vm.trees.remove(index);
    }
}
