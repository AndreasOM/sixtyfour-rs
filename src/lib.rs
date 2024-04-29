#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod engine;
// mod rotating_triangle;

//pub use rotating_triangle::RotatingTriangle;

mod property_manager;
mod state;

mod mc_guffin_window;
mod properties_window;
mod shaders_window;
mod window;

mod property_ui;
