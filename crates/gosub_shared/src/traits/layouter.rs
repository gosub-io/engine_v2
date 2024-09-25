use crate::traits::css_system::CssSystem;
use crate::traits::document::{Document, HasDocument};

pub trait HasRenderTree: Sized {
    type CssSystem: CssSystem;
    type Document: Document<Self>;
    type RenderTree: RenderTree<Self>;
}

impl<RT: HasRenderTree> HasDocument for RT {
    type CssSystem = RT::CssSystem;
    type Document = RT::Document;
}

pub trait HasLayouter: Sized {
    type Document: Document<Self>;
    type CssSystem: CssSystem;
    type RenderTree: RenderTree<Self>;
    type Layouter: Layouter<Self>;
}

impl<HL: HasLayouter> HasRenderTree for HL {
    type CssSystem = HL::CssSystem;
    type Document = HL::Document;
    type RenderTree = HL::RenderTree;
}

pub trait HasTreeDrawer: Sized {
    type Document: Document<Self>;
    type CssSystem: CssSystem;
    type RenderTree: RenderTree<Self>;
    type Layouter: Layouter<Self>;
    type TreeDrawer: TreeDrawer<Self>;
}

impl<TD: HasTreeDrawer> HasLayouter for TD {
    type Document = TD::Document;
    type CssSystem = TD::CssSystem;
    type RenderTree = TD::RenderTree;
    type Layouter = TD::Layouter;
}

pub trait Layouter<C: HasRenderTree>: Sized {
    fn do_layouter_things(&self);

    fn new() -> Self;
}

// impl<L: Layouter<R>, R: RenderTree<D>, D: Document<C>, C: CssSystem> HasLayouter for L {
//     type CssSystem = C;
//     type Document = D;
//     type RenderTree = R;
//     type Layouter = Self;
// }
//
pub trait TreeDrawer<C: HasLayouter>: Sized {
    fn do_tree_drawer_things(&self);

    fn new() -> Self;
}

// impl<T: TreeDrawer<L>, L: Layouter<R>, R: RenderTree<D>, D: Document<C>, C: CssSystem> HasTreeDrawer for T {
//     type CssSystem = C;
//     type Document = D;
//     type RenderTree = R;
//     type Layouter = L;
//     type TreeDrawer = Self;
// }

pub trait RenderTree<C: HasDocument>: Sized {
    fn do_render_tree_things(&self);

    fn new() -> Self;
}

// impl<R: RenderTree<D>, D: Document<C>, C: CssSystem> HasRenderTree for R {
//     type CssSystem = C;
//     type Document = D;
//     type RenderTree = Self;
// }

pub trait RenderBackend: Sized {
    fn do_render_backend_things(&self);

    fn new() -> Self;
}