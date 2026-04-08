use pollster::block_on;
use std::sync::Arc;
use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use super::{
    game::Game,
    renderer::Renderer,
    window::{WindowConfig, WindowHandle, create_window},
};

pub struct GameBuilder {
    window_config: Option<WindowConfig>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            window_config: None,
        }
    }

    pub fn with_window(mut self, config: WindowConfig) -> Self {
        self.window_config = Some(config);
        self
    }

    pub fn run(self, mut game: impl Game + 'static) {
        let window_config = self
            .window_config
            .expect("You must call .with_window() before .run()");
        let width = window_config.width;
        let height = window_config.height;
        let WindowHandle { window, event_loop } = create_window(window_config);

        let mut renderer = block_on(Renderer::new(Arc::clone(&window), width, height));

        game.start();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::MainEventsCleared => {
                    game.update();
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let _ = renderer.render();
                }
                _ => (),
            }
        });
    }
}
