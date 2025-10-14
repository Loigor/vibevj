mod app;
mod preview_window;
mod scene_state;

use app::VibeVJApp;
use anyhow::Result;
use winit::event_loop::EventLoop;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting VibeVJ v{}", env!("CARGO_PKG_VERSION"));

    // Create event loop
    let event_loop = EventLoop::new()?;

    // Create and run application
    let app = VibeVJApp::new()?;
    
    app.run(event_loop)?;

    Ok(())
}
