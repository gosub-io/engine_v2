use crate::traits::css_system::CssSystem;
use crate::traits::document::Document;
use crate::traits::html5_parser::{HasHtmlParser, HtmlParser};
use crate::traits::layouter::Layouter;
use crate::traits::render_backend::RenderBackend;
use crate::traits::render_tree::RenderTree;
use crate::traits::tree_drawer::{HasTreeDrawer, TreeDrawer};

pub trait ModuleConfiguration: Sized {
    type CssSystem: CssSystem;
    type Document: Document<Self>;
    type HtmlParser: HtmlParser<Self>;
    type Layouter: Layouter<Self>;
    type TreeDrawer: TreeDrawer<Self>;
    type RenderTree: RenderTree<Self>;
    type RenderBackend: RenderBackend;
}

// impl<C: ModuleConfiguration> HasCssSystem for C {
//     type CssSystem = Self::CssSystem;
// }

// impl<C: ModuleConfiguration> HasDocument for C {
//     type CssSystem = Self::CssSystem;
//     type Document = Self::Document;
// }

impl<C: ModuleConfiguration> HasHtmlParser<C> for C {
    // type CssSystem = <Self as ModuleConfiguration>::CssSystem;
    // type Document = <Self as ModuleConfiguration>::Document;
    type HtmlParser = <Self as ModuleConfiguration>::HtmlParser;
}

// impl<C: ModuleConfiguration> HasLayouter for C {
//     type Document = Self::Document;
//     type CssSystem = Self::CssSystem;
//     type RenderTree = Self::RenderTree;
//     type Layouter = Self::Layouter;
// }

impl<C: ModuleConfiguration> HasTreeDrawer for C {
    type Document = <Self as ModuleConfiguration>::Document;
    type CssSystem = <Self as ModuleConfiguration>::CssSystem;
    type RenderTree = <Self as ModuleConfiguration>::RenderTree;
    type Layouter = <Self as ModuleConfiguration>::Layouter;
    type TreeDrawer = <Self as ModuleConfiguration>::TreeDrawer;
}

// impl<C: ModuleConfiguration> HasRenderTree for C {
//     type CssSystem = Self::CssSystem;
//     type Document = Self::Document;
//     type RenderTree = Self::RenderTree;
// }