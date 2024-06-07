#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod engine;
// mod rotating_triangle;

//pub use rotating_triangle::RotatingTriangle;

mod command;
use command::Command;
mod command_queue;

mod mc_guffin_container;
use mc_guffin_container::McGuffinContainer;
mod state;

mod flow_window;
mod mc_guffin_window;
mod performance_window;
mod project_window;
mod properties_window;
mod resources_window;
mod shaders_window;
mod window;
mod window_manager;
use window_manager::WindowManager;
mod windows_menu;
use windows_menu::WindowsMenu;

mod property_ui;
mod property_ui_value;
use property_ui_value::PropertyUiValue;
mod property_ui_value_f32;
mod property_ui_value_vec2_f32;
mod property_ui_value_vec3_f32;
mod property_ui_value_vec3_f32_size4;

mod project;

mod path_helper;
mod time_series;

mod step_editor;
mod step_editor_ui;
use step_editor::StepEditor;
mod step_editor_label;
mod step_editor_program;
mod step_editor_scratch;
mod step_editor_set_uniform_f32;
mod step_editor_set_uniform_f64;
use step_editor_scratch::StepEditorScratch;

mod ui_grid;
use ui_grid::UiGrid;
mod ui_grid_cell;
use ui_grid_cell::UiGridCell;
