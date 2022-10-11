use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

/// # Window
///
/// An abstraction layer for creating a glfw window.
///
/// ## Example
/// ```
/// let mut window = Window::new(800, 600, "Window Title");
/// window.init_gl();
///
/// while !window.should_close() {
///     // Your game code goes here!
///
///     window.update;
/// }
/// ```
pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    /// Creates a new window.
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    /// Load gl functions.
    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    /// Returns whether the window should close or not.
    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    /// Poll events and swap buffers.
    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // Make sure the viewport matches the new window dimensions.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
