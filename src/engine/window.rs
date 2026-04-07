use std::sync::Arc;

use tao::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use pollster;

use crate::engine::renderer::Renderer;

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub maximizable: bool,
    pub control_flow: ControlFlow,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("App Title"),
            width: 800,
            height: 600,
            resizable: false,
            maximizable: false,
            control_flow: ControlFlow::Poll,
        }
    }
}

pub fn create_window(window_config: WindowConfig) {
    let event_loop = EventLoop::new();
    let monitor_size = event_loop
        .primary_monitor()
        .expect("Error getting monitor size")
        .size();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title(window_config.title)
            .with_inner_size(LogicalSize::new(window_config.width, window_config.height))
            .with_resizable(window_config.resizable)
            .with_maximizable(window_config.maximizable)
            .with_position(PhysicalPosition::new(
                (monitor_size.width / 2).saturating_sub(window_config.width / 2),
                (monitor_size.height / 2).saturating_sub(window_config.height / 2),
            ))
            .build(&event_loop)
            .unwrap(),
    );

    let mut renderer = pollster::block_on(Renderer::new(
        Arc::clone(&window),
        window_config.width,
        window_config.height,
    ));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = window_config.control_flow;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let _ = renderer.render();
            }
            _ => (),
        }
    });
}
