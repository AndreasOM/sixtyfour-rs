use crate::engine::gl::Gl;
use crate::engine::gl::GL_ARRAY_BUFFER;
use crate::engine::gl::GL_FALSE;
use crate::engine::gl::GL_FLOAT;
use crate::engine::gl::GL_STATIC_DRAW;
use crate::engine::gl::GL_TRIANGLE_STRIP;
use crate::engine::StepRunnerData;
use core::any::Any;

#[derive(Debug, Default)]
pub struct StepRunnerFullscreenQuad {}

impl StepRunnerFullscreenQuad {
    pub fn create_data(&self) -> Option<Box<dyn StepRunnerData>> {
        let d = StepRunnerDataFullscreenQuad::default();
        Some(Box::new(d))
    }
    pub fn run_setup(&self, gl: &Gl, data: &mut Option<Box<dyn StepRunnerData>>) {
        if let Some(data) = data {
            match data
                .as_any_mut()
                .downcast_mut::<StepRunnerDataFullscreenQuad>()
            {
                Some(data) => {
                    let mut vertex_array_id = 0;
                    gl.glGenVertexArrays(1, &mut vertex_array_id);
                    gl.glBindVertexArray(vertex_array_id);

                    let mut vertex_buffer_id = 0;
                    gl.gen_buffers(1, &mut vertex_buffer_id);
                    gl.bind_buffer(GL_ARRAY_BUFFER, vertex_buffer_id);
                    gl.check_gl_error(std::file!(), std::line!());
                    //			        self.do_data();

                    let vdata: &mut [f32] = &mut [
                        1.0, -1.0, // top right -> bottom right?
                        1.0, 1.0, // top right -> top right?
                        -1.0, -1.0, // top left -> bottom left?
                        -1.0, 1.0, // top right -> top left?
                    ];
                    let size = 4 * vdata.len();
                    gl.buffer_data(
                        GL_ARRAY_BUFFER,
                        size as isize,
                        vdata.as_ptr() as *const _,
                        GL_STATIC_DRAW,
                    );

                    data.vertex_array_id = vertex_array_id;
                    data.vertex_buffer_id = vertex_buffer_id;
                }
                None => {
                    unimplemented!();
                }
            }
        } else {
            unimplemented!();
        }
    }
    pub fn run_teardown(&self, data: &mut Option<Box<dyn StepRunnerData>>) {}
    pub fn run_render(&self, gl: &Gl, data: &Option<Box<dyn StepRunnerData>>) {
        if let Some(data) = data {
            match data.as_any().downcast_ref::<StepRunnerDataFullscreenQuad>() {
                Some(data) => {
                    gl.glBindVertexArray(data.vertex_array_id);
                    gl.bind_buffer(GL_ARRAY_BUFFER, data.vertex_buffer_id);
                    gl.enable_vertex_attrib_array(0); // 0 == pos
                    gl.vertex_attrib_pointer(0, 2, GL_FLOAT, GL_FALSE as u8, 0, core::ptr::null());
                    gl.draw_arrays(GL_TRIANGLE_STRIP, 0, 4);
                }
                None => {
                    unimplemented!();
                }
            }
        } else {
            unimplemented!();
        }
    }
}

#[derive(Debug, Default)]
struct StepRunnerDataFullscreenQuad {
    vertex_array_id: u32,
    vertex_buffer_id: u32,
}

impl StepRunnerData for StepRunnerDataFullscreenQuad {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
