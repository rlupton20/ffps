extern crate glutin;

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
}

impl Gl {
    pub fn draw_frame(&self, color: [f32; 4]) {
        unsafe {
            self.gl.ClearColor(color[0], color[1], color[2], color[3]);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

const VS_SRC: &'static [u8] = b"                                                
#version 100                                                                    
precision mediump float;                                                        
                                                                                
attribute vec2 position;                                                        
attribute vec3 color;                                                           
                                                                                
varying vec3 v_color;                                                           
                                                                                
void main() {                                                                   
    gl_Position = vec4(position, 0.0, 1.0);                                     
    v_color = color;                                                            
}                                                                               
\0";

const FS_SRC: &'static [u8] = b"                                                
#version 100                                                                    
precision mediump float;                                                        
                                                                                
varying vec3 v_color;                                                           
                                                                                
void main() {                                                                   
    gl_FragColor = vec4(v_color, 1.0);                                          
}                                                                               
\0";

static VERTEX_DATA: [f32; 15] = [                                               
    -0.5, -0.5,  1.0,  0.0,  0.0,                                               
     0.0,  0.5,  0.0,  1.0,  0.0,                                               
     0.5, -0.5,  0.0,  0.0,  1.0,                                               
];

unsafe fn shaders(gl: &gl::Gl) {
    let vs = gl.CreateShader(gl::VERTEX_SHADER);                               
    gl.ShaderSource(                                                           
        vs,                                                                    
        1,                                                                     
        [VS_SRC.as_ptr() as *const _].as_ptr(),                                
        std::ptr::null(),                                                      
    );                                                                         
    gl.CompileShader(vs);                                                      
                                                                               
    let fs = gl.CreateShader(gl::FRAGMENT_SHADER);                             
    gl.ShaderSource(                                                           
        fs,                                                                    
        1,                                                                     
        [FS_SRC.as_ptr() as *const _].as_ptr(),                                
        std::ptr::null(),                                                      
    );                                                                         
    gl.CompileShader(fs);                                                      
                                                                               
    let program = gl.CreateProgram();                                          
    gl.AttachShader(program, vs);                                              
    gl.AttachShader(program, fs);                                              
    gl.LinkProgram(program);                                                   
    gl.UseProgram(program);                                                    
                                                                               
    let mut vb = std::mem::uninitialized();                                    
    gl.GenBuffers(1, &mut vb);                                                 
    gl.BindBuffer(gl::ARRAY_BUFFER, vb);                                       
    gl.BufferData(                                                             
        gl::ARRAY_BUFFER,                                                      
        (VERTEX_DATA.len() * std::mem::size_of::<f32>())                       
            as gl::types::GLsizeiptr,                                          
        VERTEX_DATA.as_ptr() as *const _,                                      
        gl::STATIC_DRAW,                                                       
    );                                                                         
                                                                               
    if gl.BindVertexArray.is_loaded() {                                        
        let mut vao = std::mem::uninitialized();                               
        gl.GenVertexArrays(1, &mut vao);                                       
        gl.BindVertexArray(vao);                                               
    }                                                                          
                                                                               
    let pos_attrib =                                                           
        gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);  
    let color_attrib =                                                         
        gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);        
    gl.VertexAttribPointer(                                                    
        pos_attrib as gl::types::GLuint,                                       
        2,                                                                     
        gl::FLOAT,                                                             
        0,                                                                     
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,                  
        std::ptr::null(),                                                      
    );                                                                         
    gl.VertexAttribPointer(                                                    
        color_attrib as gl::types::GLuint,                                     
        3,                                                                     
        gl::FLOAT,                                                          
        0,                                                                  
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,               
        (2 * std::mem::size_of::<f32>()) as *const () as *const _,          
    );                                                                      
    gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);            
    gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);
}

fn gl(context: &glutin::Context<glutin::PossiblyCurrent>) -> Gl {
    Gl {
        gl: {
            let gl = gl::Gl::load_with(
                |ptr| context.get_proc_address(ptr) as *const _
            );

            unsafe { shaders(&gl) };

            gl
        },
    }
}

struct State {
    x: usize,
    y: usize,
}

fn main() -> Result<(), ()> {
    let mut state = State { x: 100, y: 100 };
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

    let gl = gl(&windowed_context.context());

    el.run_forever(move |e: glutin::Event| {
        println!("got event {:?}", e);

        match e {
            glutin::Event::WindowEvent { ref event, .. } => match event {
                glutin::WindowEvent::CloseRequested => {
                    glutin::ControlFlow::Break
                },
                glutin::WindowEvent::Refresh => {
                    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    windowed_context.swap_buffers().expect("Swapbuf fail");
                    glutin::ControlFlow::Continue
                },
                glutin::WindowEvent::Resized(log_size) => {
                    windowed_context.resize(
                        log_size.to_physical(
                            windowed_context.window().get_hidpi_factor()
                        )
                    );
                    glutin::ControlFlow::Continue
                },
                glutin::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                } => {
                    glutin::ControlFlow::Continue
                },
                glutin::WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    modifiers
                } => {
                    glutin::ControlFlow::Continue
                },
                glutin::WindowEvent::CursorMoved {
                    device_id,
                    position,
                    modifiers,
                } => {
                    glutin::ControlFlow::Continue
                },
                glutin::WindowEvent::HiDpiFactorChanged(_) |
                glutin::WindowEvent::Focused(_) |
                glutin::WindowEvent::Moved(_) |
                glutin::WindowEvent::TouchpadPressure { .. } |
                glutin::WindowEvent::AxisMotion { .. } |
                glutin::WindowEvent::ReceivedCharacter(_) |
                glutin::WindowEvent::MouseWheel { .. } |
                glutin::WindowEvent::CursorEntered { .. } |
                glutin::WindowEvent::CursorLeft { .. } => {
                    glutin::ControlFlow::Continue
                },
                e => panic!("{:?}", e),
            },
            glutin::Event::DeviceEvent { ref event, .. } => match event {
                glutin::DeviceEvent::Motion { .. } |
                glutin::DeviceEvent::Added { ..  } |
                glutin::DeviceEvent::Removed { .. } |
                glutin::DeviceEvent::MouseMotion { .. } |
                glutin::DeviceEvent::MouseWheel { .. } => {
                    glutin::ControlFlow::Continue
                },
                e => panic!("{:?}", e),
            },
            e => panic!("{:?}", e),
        }
    });

    Ok(())
}
