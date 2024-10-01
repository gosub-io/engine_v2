use crate::traits::css_system::HasCssSystem;
use crate::traits::document::HasDocument;
use crate::traits::html5_parser::HasHtmlParser;
use crate::traits::layouter::HasLayouter;
use crate::traits::render_backend::HasRenderBackend;
use crate::traits::render_tree::HasRenderTree;
use crate::traits::tree_drawer::HasTreeDrawer;

pub trait ModuleConfiguration:
    Sized
    + HasCssSystem
    + HasDocument
    + HasHtmlParser
    + HasLayouter
    + HasRenderTree
    + HasTreeDrawer
    + HasRenderBackend
{}