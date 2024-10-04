use std::fmt::Display;
use crate::traits::render_tree::HasRenderTree;

pub trait Box {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn title(&self) -> &str;
    fn z_index(&self) -> i32;
}

pub trait Size {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
}

pub trait HasLayouter: Sized  + HasRenderTree {
    type Layouter: Layouter<Self>;
}

pub trait Layouter<C: HasRenderTree>: Sized {
    fn from_render_tree(render_tree: C::RenderTree, size: impl Size) -> Self;
    fn get_boxes(&self) -> &Vec<impl Box + Display>;
}