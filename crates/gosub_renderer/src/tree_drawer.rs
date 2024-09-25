use gosub_shared::traits::layouter::HasLayouter;
use gosub_shared::traits::tree_drawer::TreeDrawer;

pub struct MyTreeDrawer;

impl<C: HasLayouter> TreeDrawer<C> for MyTreeDrawer {
    fn do_tree_drawer_things(&self) {
        println!("Doing tree drawer things");
    }

    fn new() -> Self {
        Self
    }
}
