#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
pub mod fps_limiter;
pub mod game_option;
pub mod plugin;
pub mod scale_factor;
pub mod show_fps;
