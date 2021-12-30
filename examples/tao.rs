fn main() {
  use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
  };
  use tauri_plugin_shadows::Shadows;

  let event_loop = EventLoop::new();

  let window = WindowBuilder::new()
    .with_decorations(false)
    .with_transparent(true)
    .build(&event_loop)
    .unwrap();

  window.set_shadow(true);

  window.set_title("A fantastic window!");

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => window.set_shadow(false),
      _ => (),
    }
  });
}
