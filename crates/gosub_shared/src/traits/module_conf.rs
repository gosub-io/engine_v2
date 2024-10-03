use crate::traits::css_system::{HasCssParser, HasCssSystem};
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
    + HasCssParser
    + HasHtmlParser
    + HasLayouter
    + HasRenderTree
    + HasTreeDrawer
    + HasRenderBackend
{}