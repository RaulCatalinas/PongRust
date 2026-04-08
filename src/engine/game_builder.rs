use pollster::block_on;
use std::sync::Arc;
use tao::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
};

use crate::engine::{cursor, input, ui::text};

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
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        text::shutdown();
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        input::mouse::on_moved(position.x as f32, position.y as f32);
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if button == MouseButton::Left {
                            input::mouse::on_button(state == ElementState::Pressed);
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        input::keyboard::on_key(
                            event.physical_key,
                            event.state == ElementState::Pressed,
                        );
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    game.update();
                    window.set_cursor_icon(cursor::get());
                    cursor::reset();
                    window.request_redraw();
                    input::flush();
                }
                Event::RedrawRequested(_) => {
                    let _ = renderer.render();
                }
                _ => (),
            }
        });
    }
}
