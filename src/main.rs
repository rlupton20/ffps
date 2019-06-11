extern crate glutin;


fn main() -> Result<(),  ()> {
    let mut el = glutin::EventsLoop::new();

    let wb = glutin::WindowBuilder::new()
        .with_title("FFPS - Fuck-yeah FPS")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));

    let windowed_context = {
        let ctxt = glutin::ContextBuilder::new()
            .build_windowed(wb, &el)
            .expect("Create context fail");

        let ctxt = unsafe { ctxt.make_current().expect("Make current fail") };

        ctxt
    };

    el.run_forever(|e: glutin::Event| {
        println!("got event {:?}", e);
        match e {
            glutin::Event::WindowEvent{ ref event, ..} if is_close_request(event) =>
                glutin::ControlFlow::Break,
            _ => glutin::ControlFlow::Continue
        }
    });

    Ok(())
}

fn is_close_request(e: &glutin::WindowEvent)-> bool {
    *e == glutin::WindowEvent::CloseRequested
}
