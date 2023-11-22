use std::num::NonZeroU32;
use std::rc::Rc;
use verle::colors::{GRAY, GREEN, RED};
use verle::Canvas;
use verle::Pos;
use verle::Size;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Icon, WindowBuilder};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("verle")
            .with_inner_size(PhysicalSize::new(640, 480))
            // .with_resizable(false)
            .with_window_icon(Icon::from_rgba(vec![0x00, 0xff, 0x00, 0xff], 1, 1).ok())
            .build(&event_loop)
            .unwrap(),
    );
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id,
            } if window_id == window.id() => {
                let size = window.inner_size();
                let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                else {
                    return;
                };
                surface.resize(width, height).unwrap();

                let mut canvas = Canvas::new(GRAY, Size::new(width.into(), height.into()));

                let origin = Pos::new(200, 200);
                let radius = 200u32;

                canvas.draw_circle(origin, radius, RED, GREEN);
                canvas.draw_circle(Pos::new(100, 100), 100, RED, GREEN);
                canvas.draw_circle(Pos::new(275, 100), 100, RED, GREEN);
                canvas.draw_circle(Pos::new(100, 325), 100, RED, GREEN);
                canvas.draw_circle(Pos::new(325, 275), 100, RED, GREEN);
                canvas.draw_circle(Pos::new(150, 150), 1, RED, GREEN);
                canvas.draw_circle(Pos::new(150, 100), 0, RED, GREEN);

                let mut buffer = surface.buffer_mut().unwrap();
                window.pre_present_notify();
                canvas.flush(&mut buffer).unwrap();
                buffer.present().unwrap();

                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        })
        .unwrap();
}
