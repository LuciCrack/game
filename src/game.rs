mod renderer;
mod player;

use log::{debug, error, warn};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use renderer::{Renderer, object::Object};
use player::Player;


pub struct Game<'a> {
    renderer: Renderer<'a>,
    pub player: Player,
    // Input handler 
    // Player
    // others..
}

impl<'a> Game<'a>{
    fn new(window: Window) -> Self {
        let renderer = Renderer::new(window);
        let object = Object::new(&renderer);
        let player = Player::new(object);
        Self {
            renderer,
            player
        }
    }
}

pub struct GameApplication<'a> {
    state: Option<Game<'a>>,
}

impl<'a> GameApplication<'a> {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl<'a> ApplicationHandler for GameApplication<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_att = Window::default_attributes().with_title("Mario");
        let window = event_loop.create_window(window_att).unwrap();
        self.state = Some(Game::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let game = self.state.as_mut().unwrap();
        let window = game.renderer.window();

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    game.renderer.resize(physical_size);
                    self.about_to_wait(event_loop);
                }
                WindowEvent::RedrawRequested => {
                    match game.renderer.render(&game.player) {
                        Ok(_) => {} // Everything is Ok
                        // Reconfigure surface if lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            let size = game.renderer.size;
                            debug!(
                                "Surface lost or outdated, reconfiguring with size: {}x{}",
                                size.width, size.height
                            );
                            game.renderer.resize(size);
                        }
                        // Out of memory, quit
                        Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                            error!("OutOfMemory");
                            event_loop.exit();
                        }
                        // Timeout, when a frame takes to long to present
                        Err(wgpu::SurfaceError::Timeout) => {
                            warn!("Surface Timeout");
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let renderer = &self.state.as_mut().unwrap().renderer;
        let window = renderer.window();
        window.request_redraw();
    }
}
