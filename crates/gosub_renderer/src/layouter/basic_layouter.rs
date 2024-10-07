use std::fmt::Display;
use gosub_html5::document::tree_iterator::TreeIterator;
use gosub_shared::traits::document::Document;
use gosub_shared::traits::layouter::Layouter;
use gosub_shared::traits::layouter::Size as SizeTrait;
use gosub_shared::traits::layouter::Box as BoxTrait;
use gosub_shared::traits::node::{ElementData, Node, TextData};
use gosub_shared::traits::render_tree::{HasRenderTree, RenderTree};

#[derive(Debug, Clone)]
pub struct Box {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub z_index: i32,
}

impl Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box[ x: {}, y: {}, width: {}, height: {}, title: {}, z_index: {} ]", self.x, self.y, self.width, self.height, self.title, self.z_index)
    }
}

impl BoxTrait for Box {
    fn x(&self) -> f32 {
        self.x as f32
    }

    fn y(&self) -> f32 {
        self.y as f32
    }

    fn width(&self) -> f32 {
        self.width as f32
    }

    fn height(&self) -> f32 {
        self.height as f32
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }
}

#[derive(Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl SizeTrait for Size {
    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }
}

pub struct BasicLayouter<C: HasRenderTree> {
    boxes: Vec<Box>,
    marker: std::marker::PhantomData<C>,
}

impl<C: HasRenderTree> Layouter<C> for BasicLayouter<C> {
    fn from_render_tree(render_tree: C::RenderTree, size: impl SizeTrait) -> Self {
        let root = Box {
            x: 0,
            y: 0,
            width: size.width() as i32,
            height: size.height() as i32,
            title: "root".to_string(),
            z_index: 0,
        };

        let mut parent_box = root.clone();
        let mut boxes = vec![root];

        let binding = render_tree.get_handle();
        let binding = binding.get();

        for node_id in TreeIterator::new(render_tree.get_handle().clone()) {
            let node = binding.get_node(node_id).unwrap();

            if let Some(element) = node.get_element_data() {
                let box_ = Box {
                    x: parent_box.x + 10,
                    y: parent_box.y + 10,
                    width: parent_box.width / 3 * 2,
                    height: parent_box.height / 3 * 2,
                    title: element.name().to_string(),
                    z_index: 0,
                };

                parent_box = box_.clone();
                boxes.push(box_);
            }

            if let Some(text) = node.get_text_data() {
                let box_ = Box {
                    x: parent_box.x + 2,
                    y: parent_box.y + 2,
                    width: parent_box.width - 2,
                    height: parent_box.height - 2,
                    title: text.content().to_string(),
                    z_index: 0,
                };

                boxes.push(box_);
            }
        }

        Self {
            boxes,
            marker: std::marker::PhantomData,
        }
    }

    fn get_boxes(&self) -> &Vec<impl BoxTrait + Display> {
        &self.boxes
    }
}