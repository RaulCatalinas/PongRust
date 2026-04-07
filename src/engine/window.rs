use tao::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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
    let window = WindowBuilder::new()
        .with_title(window_config.title)
        .with_inner_size(LogicalSize::new(window_config.width, window_config.height))
        .with_resizable(window_config.resizable)
        .with_maximizable(window_config.maximizable)
        .with_position(PhysicalPosition::new(
            (monitor_size.width / 2).saturating_sub(window_config.width / 2),
            (monitor_size.height / 2).saturating_sub(window_config.height / 2),
        ))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = window_config.control_flow;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            }
            _ => (),
        }
    });
}
