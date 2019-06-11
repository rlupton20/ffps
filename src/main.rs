extern crate glutin;


fn main() {
    let mut el = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_title("FFPS - Fuck-yeah FPS")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();

    el.run_forever(|e: glutin::Event| {
        println!("got event {:?}", e);
        glutin::ControlFlow::Continue
    });

}
