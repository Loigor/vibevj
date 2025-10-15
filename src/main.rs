mod app;
mod preview_window;
mod scene_state;

use app::{VibeVJApp, AppEvent};
use anyhow::Result;
use winit::event_loop::EventLoop;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting VibeVJ v{}", env!("CARGO_PKG_VERSION"));

    // Create event loop with custom events
    let event_loop = EventLoop::<AppEvent>::with_user_event().build()?;
    let event_loop_proxy = event_loop.create_proxy();

    // Spawn animation timer thread that runs independently of window events
    // This ensures animation continues even during window dragging
    thread::spawn(move || {
        let target_fps = 60.0;
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps);
        
        loop {
            thread::sleep(frame_duration);
            
            // Send animation tick event to main event loop
            if event_loop_proxy.send_event(AppEvent::AnimationTick).is_err() {
                // Event loop has been closed, exit thread
                break;
            }
        }
    });

    // Create and run application
    let app = VibeVJApp::new()?;
    
    app.run(event_loop)?;

    Ok(())
}
