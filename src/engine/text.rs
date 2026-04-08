use std::cell::RefCell;

use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache,
    TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use wgpu::{Device, MultisampleState, Queue, RenderPass, TextureFormat};

struct TextConfig {
    text: String,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
}

struct EngineText {
    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: Viewport,
    atlas: TextAtlas,
    renderer: TextRenderer,
    texts: Vec<(Buffer, TextConfig)>,
}

impl EngineText {
    fn new(device: &Device, queue: &Queue, format: TextureFormat) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let renderer = TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);

        Self {
            font_system,
            swash_cache,
            viewport,
            atlas,
            renderer,
            texts: vec![],
        }
    }

    fn add(&mut self, config: TextConfig) {
        let mut buffer = Buffer::new(
            &mut self.font_system,
            Metrics::new(config.size, config.size * 1.2),
        );
        buffer.set_size(&mut self.font_system, None, None);
        buffer.set_text(
            &mut self.font_system,
            &config.text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut self.font_system, false);
        self.texts.push((buffer, config));
    }

    fn prepare(&mut self, device: &Device, queue: &Queue, width: u32, height: u32) {
        self.viewport.update(queue, Resolution { width, height });

        let text_areas: Vec<TextArea> = self
            .texts
            .iter()
            .map(|(buffer, config)| TextArea {
                buffer,
                left: config.x,
                top: config.y,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: width as i32,
                    bottom: height as i32,
                },
                default_color: config.color,
                custom_glyphs: &[],
            })
            .collect();

        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                text_areas,
                &mut self.swash_cache,
            )
            .unwrap();
    }

    fn clear(&mut self) {
        self.texts.clear();
    }
}

thread_local! {
    static ENGINE_TEXT: RefCell<Option<EngineText>> = RefCell::new(None);
}

pub(crate) fn init(device: &Device, queue: &Queue, format: TextureFormat) {
    ENGINE_TEXT.with(|t| {
        *t.borrow_mut() = Some(EngineText::new(device, queue, format));
    });
}

pub(crate) fn prepare(device: &Device, queue: &Queue, width: u32, height: u32) {
    ENGINE_TEXT.with(|t| {
        if let Some(engine_text) = t.borrow_mut().as_mut() {
            engine_text.prepare(device, queue, width, height);
        }
    });
}

pub(crate) fn render_in_pass<'a>(render_pass: &mut RenderPass<'a>) {
    ENGINE_TEXT.with(|t| {
        let mut borrowed = t.borrow_mut();
        if let Some(engine_text) = borrowed.as_mut() {
            engine_text
                .renderer
                .render(&engine_text.atlas, &engine_text.viewport, render_pass)
                .unwrap();
        }
    });
}
pub(crate) fn clear() {
    ENGINE_TEXT.with(|t| {
        if let Some(engine_text) = t.borrow_mut().as_mut() {
            engine_text.clear();
        }
    });
}

pub fn draw(text: &str, x: f32, y: f32, size: f32) {
    ENGINE_TEXT.with(|t| {
        if let Some(engine_text) = t.borrow_mut().as_mut() {
            engine_text.add(TextConfig {
                text: text.to_string(),
                x,
                y,
                size,
                color: Color::rgb(255, 255, 255),
            });
        }
    });
}

pub fn draw_colored(text: &str, x: f32, y: f32, size: f32, color: Color) {
    ENGINE_TEXT.with(|t| {
        if let Some(engine_text) = t.borrow_mut().as_mut() {
            engine_text.add(TextConfig {
                text: text.to_string(),
                x,
                y,
                size,
                color,
            });
        }
    });
}

pub(crate) fn shutdown() {
    ENGINE_TEXT.with(|t| {
        *t.borrow_mut() = None;
    });
}
