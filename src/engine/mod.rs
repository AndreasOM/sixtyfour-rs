mod mc_guffin;
pub use mc_guffin::McGuffin;

mod gl;
mod gl_wrapper_macro;

mod shader_source;
pub use shader_source::ShaderSource;
mod pipeline;
use pipeline::Pipeline;
mod uniform;
use uniform::Uniform;
pub use uniform::UniformType;
mod uniform_manager;
pub use uniform_manager::UniformManager;
