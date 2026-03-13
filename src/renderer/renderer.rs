use crate::component::ComponentNode;

#[derive(Debug, Clone)]
pub struct RenderFrame {
    pub window_title: String,
    pub root: Option<ComponentNode>,
}

#[derive(Debug, Default)]
pub struct Renderer {
    rendered_frames: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, frame: RenderFrame) {
        #[cfg(feature = "sdl-backend")]
        {
            self.render_with_sdl(frame);
        }

        #[cfg(not(feature = "sdl-backend"))]
        {
            self.render_headless(frame);
        }
    }

    pub fn rendered_frames(&self) -> usize {
        self.rendered_frames
    }

    #[cfg(not(feature = "sdl-backend"))]
    fn render_headless(&mut self, _frame: RenderFrame) {
        self.rendered_frames += 1;
    }

    #[cfg(feature = "sdl-backend")]
    fn render_with_sdl(&mut self, frame: RenderFrame) {
        let _ = sdl3::hint::set("ICY_WINDOW_TITLE", &frame.window_title);
        self.rendered_frames += 1;
    }

    #[cfg(feature = "sdl-backend")]
    pub fn render_with_canvas(
        &mut self,
        frame: RenderFrame,
        canvas: &mut sdl3::render::WindowCanvas,
        font: Option<&sdl3::ttf::Font<'_>>,
    ) {
        use sdl3::pixels::Color;

        let _ = sdl3::hint::set("ICY_WINDOW_TITLE", &frame.window_title);

        canvas.set_draw_color(Color::RGB(20, 24, 32));
        canvas.clear();

        let mut y = 24i32;
        if let Some(root) = frame.root {
            Self::render_node_recursive(canvas, &root, 0, &mut y, font);
        }

        let _ = canvas.present();
        self.rendered_frames += 1;
    }

    #[cfg(feature = "sdl-backend")]
    fn render_node_recursive(
        canvas: &mut sdl3::render::WindowCanvas,
        node: &ComponentNode,
        depth: usize,
        y: &mut i32,
        font: Option<&sdl3::ttf::Font<'_>>,
    ) {
        use crate::component::ElementType;
        use sdl3::pixels::Color;
        use sdl3::rect::Rect;

        let indent = (depth as i32) * 20;
        let x = 24 + indent;
        let width = (760i32 - indent).max(160) as u32;

        match node.element {
            ElementType::Root => {
                if depth > 0 {
                    let outer = Rect::new(x, *y, width, 26);
                    let inner = Rect::new(x + 4, *y + 4, width.saturating_sub(8), 18);
                    canvas.set_draw_color(Color::RGB(36, 40, 50));
                    let _ = canvas.fill_rect(outer);
                    canvas.set_draw_color(Color::RGB(70, 76, 91));
                    let _ = canvas.fill_rect(inner);
                    *y += 34;
                }

                for child in &node.children {
                    Self::render_node_recursive(canvas, child, depth + 1, y, font);
                }
            }
            ElementType::H1 | ElementType::Paragraph | ElementType::Button => {
                let (height, color) = match node.element {
                    ElementType::H1 => (64u32, Color::RGB(62, 112, 231)),
                    ElementType::Paragraph => (44u32, Color::RGB(55, 163, 127)),
                    ElementType::Button => (52u32, Color::RGB(224, 153, 53)),
                    ElementType::Root => unreachable!(),
                };

                let outer = Rect::new(x, *y, width, height);
                let inner = Rect::new(
                    x + 5,
                    *y + 5,
                    width.saturating_sub(10),
                    height.saturating_sub(10),
                );

                canvas.set_draw_color(Color::RGB(36, 40, 50));
                let _ = canvas.fill_rect(outer);
                canvas.set_draw_color(color);
                let _ = canvas.fill_rect(inner);

                if let (Some(text_node), Some(font_ref)) = (&node.text, font) {
                    Self::draw_text(
                        canvas,
                        font_ref,
                        &text_node.text,
                        x + 12,
                        *y + 10,
                        width.saturating_sub(24),
                    );
                }

                *y += height as i32 + 12;
            }
        }
    }

    #[cfg(feature = "sdl-backend")]
    fn draw_text(
        canvas: &mut sdl3::render::WindowCanvas,
        font: &sdl3::ttf::Font<'_>,
        text: &str,
        x: i32,
        y: i32,
        max_width: u32,
    ) {
        use sdl3::pixels::Color;
        use sdl3::rect::Rect;
        use sdl3::render::TextureQuery;

        let Ok(surface) = font.render(text).blended(Color::RGB(245, 248, 255)) else {
            return;
        };

        let texture_creator = canvas.texture_creator();
        let Ok(texture) = texture_creator.create_texture_from_surface(&surface) else {
            return;
        };

        let TextureQuery { width, height, .. } = texture.query();
        let target_w = width.min(max_width).max(1);
        let target_h = height.max(1);
        let target = Rect::new(x, y, target_w, target_h);

        let _ = canvas.copy(&texture, None, Some(target.into()));
    }
}
