#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod engine;
// mod rotating_triangle;

//pub use rotating_triangle::RotatingTriangle;

mod state;

mod mc_guffin_window;
mod project_window;
mod properties_window;
mod shaders_window;
mod window;

mod property_ui;
mod property_ui_value;
use property_ui_value::PropertyUiValue;
mod property_ui_value_f32;
mod property_ui_value_vec3_f32;

mod project;
