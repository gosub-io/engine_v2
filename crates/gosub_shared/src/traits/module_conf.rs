use crate::traits::css_system::CssSystem;
use crate::traits::document::Document;
use crate::traits::html5_parser::HtmlParser;
use crate::traits::layouter::{Layouter, RenderBackend, RenderTree, TreeDrawer};

pub trait ModuleConfiguration: Sized {
    type CssSystem: CssSystem;
    type Document: Document<Self>;
    type HtmlParser: HtmlParser<Self>;
    type Layouter: Layouter<Self>;
    type TreeDrawer: TreeDrawer<Self>;
    type RenderTree: RenderTree<Self>;
    type RenderBackend: RenderBackend;
}