use gosub_shared::traits::renderer::{HasLayouter, TreeDrawer};

struct MyTreeDrawer;

impl<C: HasLayouter> TreeDrawer<C> for MyTreeDrawer {
    fn do_tree_drawer_things(&self) {
        println!("Doing tree drawer things");
    }

    fn new() -> Self {
        Self
    }
}
