#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
pub mod fps_limiter;
pub mod option_name;
pub mod scale_factor;
pub mod show_fps;
#[cfg(not(target_arch = "wasm32"))]
pub mod window_mode;
