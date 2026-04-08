use std::sync::Arc;
use tao::{
    dpi::{LogicalSize, PhysicalPosition},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub maximizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Game Title"),
            width: 800,
            height: 600,
            resizable: true,
            maximizable: true,
        }
    }
}

pub(crate) struct WindowHandle {
    pub window: Arc<Window>,
    pub event_loop: EventLoop<()>,
}

pub(crate) fn create_window(config: WindowConfig) -> WindowHandle {
    let event_loop = EventLoop::new();
    let monitor_size = event_loop
        .primary_monitor()
        .expect("Error getting monitor size")
        .size();

    let window = Arc::new(
        WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_resizable(config.resizable)
            .with_maximizable(config.maximizable)
            .with_position(PhysicalPosition::new(
                (monitor_size.width / 2).saturating_sub(config.width / 2),
                (monitor_size.height / 2).saturating_sub(config.height / 2),
            ))
            .build(&event_loop)
            .unwrap(),
    );

    WindowHandle { window, event_loop }
}
