extern crate glutin;

fn main() {
    let window = glutin::Window::new().unwrap();


    'main:loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
