mod game;

use log::info;
use winit::event_loop::EventLoop;

use game::GameApplication;

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let mut window_state = GameApplication::new();
    info!("Running app!");
    let _ = event_loop.run_app(&mut window_state);
}

