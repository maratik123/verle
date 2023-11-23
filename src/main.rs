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
                let (width, height) = (u32::from(width), u32::from(height));

                let mut canvas = Canvas::new(GRAY, Size::new(width, height));

                canvas.draw_circle(Pos::new(0, 0), 1, RED, GREEN);
                canvas.draw_circle(Pos::new(width - 1, 0), 1, RED, GREEN);
                canvas.draw_circle(Pos::new(0, height - 1), 1, RED, GREEN);
                canvas.draw_circle(Pos::new(width - 1, height - 1), 1, RED, GREEN);
                canvas.draw_line(Pos::new(0, 0), Pos::new(width, height), RED);
                canvas.draw_line(Pos::new(width + 10, height), Pos::new(10, 0), RED);
                canvas.draw_line(Pos::new(width, 0), Pos::new(0, height), RED);
                canvas.draw_line(Pos::new(10, height), Pos::new(width + 10, 0), RED);
                canvas.draw_line(Pos::new(100, 100), Pos::new(100, 200), GREEN);
                canvas.draw_line(Pos::new(100, 100), Pos::new(200, 100), GREEN);
                canvas.draw_line(Pos::new(200, 200), Pos::new(100, 200), GREEN);
                canvas.draw_line(Pos::new(200, 200), Pos::new(200, 100), GREEN);

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
