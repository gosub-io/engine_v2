use gosub_shared::traits::layouter::Box;
use gosub_shared::traits::layouter::{HasLayouter, Layouter};
use gosub_shared::traits::render_backend::RenderBackend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui::DefaultTerminal;

/// This is an example backend that uses the `ratatui` crate to render the layout. It gets the box
/// layout from the layouter, and will generate boxes onto the screen based on the layout.

pub struct MyRatatuiRenderBackend<C: HasLayouter> {
    terminal: DefaultTerminal,
    layout: C::Layouter,
}

impl<C: HasLayouter> RenderBackend<C> for MyRatatuiRenderBackend<C> {
    fn from_layouter(layout: C::Layouter) -> Self {
        let mut terminal = ratatui::init();
        let _ = terminal.clear();

        Self { terminal, layout }
    }

    fn render_scene(&mut self) {
        let _ = self.terminal.draw(|frame| {
            // Calculate scaling factor between the terminal size, and the layouter size
            let root_box = self.layout.get_boxes().first().unwrap();

            let scale_x = frame.area().width as f32 / root_box.width();
            let scale_y = frame.area().height as f32 / root_box.height();

            // Render all boxes
            for box_ in self.layout.get_boxes() {
                let x = (box_.x() * scale_x) as u16;
                let y = (box_.y() * scale_y) as u16;
                let width = (box_.width() * scale_x) as u16;
                let height = (box_.height() * scale_y) as u16;

                let b = Block::default()
                    .borders(Borders::ALL)
                    .title_bottom(box_.title());

                frame.render_widget(b, Rect::new(x, y, width, height));
            }
        });
    }
}
