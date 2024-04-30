use super::gl::*;
#[allow(non_camel_case_types)]
pub type void = ();

/*
    pub fn get_error(&self) -> core::ffi::c_uint {
        unsafe {
            let fn_p =
                transmute::<*const c_void, extern "system" fn() -> c_uint>(self.glfp_get_error.f);
            fn_p()
        }
    }
*/
/*
        self.glfps.glCreateShader
            .load(get_proc_address, c"glCreateShader")?;

*/
/*
macro_rules! generate_gl_load {
        ($name:ident) => {
            self.glfps.$name
                .load(get_proc_address, stringify!($name) /*c"$name"*/ )?;
        }
}
*/

macro_rules! create_gl_wrapper {
    	//type ident( type ident )

    	//($return_type:ty,$name:ident) => {
    	($return_type:ident $name:ident( void )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn() -> $return_type>(self.glfps.$name.f)()
    			}
    		}
    	};
    	($return_type:ident $name:ident( $t0:ident $p0:ident )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self, $p0: $t0) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn( $t0 ) -> $return_type>(self.glfps.$name.f)( $p0 )
    			}
    		}
    	};
    	($return_type:ident $name:ident( $t0:ident $p0:ident, $t1:ident $p1:ident )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self, $p0: $t0, $p1: $t1) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn( $t0, $t1 ) -> $return_type>(self.glfps.$name.f)( $p0, $p1 )
    			}
    		}
    	};
    	($return_type:ident $name:ident( $t0:ident $p0:ident, $t1:ident $p1:ident, $t2:ident $p2:ident )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self, $p0: $t0, $p1: $t1, $p2: $t2) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn( $t0, $t1, $t2 ) -> $return_type>(self.glfps.$name.f)( $p0, $p1, $p2 )
    			}
    		}
    	};
        ($return_type:ident $name:ident(
            $t0:ident $p0:ident,
            $t1:ident $p1:ident,
            $t2:ident $p2:ident,
            const $t3:ident *$p3:ident
        )) => {
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            pub /*unsafe*/ fn $name(
                &self,
                $p0: $t0,
                $p1: $t1,
                $p2: $t2,
                $p3: *const $t3,
                ) -> $return_type {
                unsafe {
                    core::mem::transmute::<*const core::ffi::c_void, extern "system" fn(
                        $t0,
                        $t1,
                        $t2,
                        *const $t3,
                        ) -> $return_type>(self.glfps.$name.f)( $p0, $p1, $p2, $p3 )
                }
            }
        };
    	($return_type:ident $name:ident(
    		$t0:ident $p0:ident,
    		$t1:ident $p1:ident,
    		$t2:ident $p2:ident,
			$t3:ident *$p3:ident,
			$t4:ident *$p4:ident,
			$t5:ident *$p5:ident,
			$t6:ident *$p6:ident
    	)) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(
    			&self,
    			$p0: $t0,
    			$p1: $t1,
    			$p2: $t2,
    			$p3: *mut $t3,
    			$p4: *mut $t4,
    			$p5: *mut $t5,
    			$p6: *mut $t6,
    			) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn(
    					$t0,
    					$t1,
    					$t2,
    					*mut $t3,
    					*mut $t4,
    					*mut $t5,
    					*mut $t6,
    					) -> $return_type>(self.glfps.$name.f)( $p0, $p1, $p2, $p3, $p4, $p5, $p6 )
    			}
    		}
    	};
    	($return_type:ident $name:ident( $t0:ident $p0:ident, const $t1:ident *$p1:ident )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self, $p0: $t0, $p1: *const $t1) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn( $t0, *const $t1 ) -> $return_type>(self.glfps.$name.f)( $p0, $p1 )
    			}
    		}
    	};
    	($return_type:ident $name:ident( $t0:ident $p0:ident, $t1:ident *$p1:ident )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self, $p0: $t0, $p1: *mut $t1) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn( $t0, *mut $t1 ) -> $return_type>(self.glfps.$name.f)( $p0, $p1 )
    			}
    		}
    	};
    	($return_type:ident $name:ident( $t0:ident $p0:ident, $t1:ident $p1:ident, $t2:ident *$p2:ident )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self, $p0: $t0, $p1: $t1, $p2: *mut $t2) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn( $t0, $t1, *mut $t2 ) -> $return_type>(self.glfps.$name.f)( $p0, $p1, $p2 )
    			}
    		}
    	};
    	($return_type:ident $name:ident( $t0:ident $p0:ident, $t1:ident $p1:ident, $t2:ident *$p2:ident, $t3:ident *$p3:ident )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self, $p0: $t0, $p1: $t1, $p2: *mut $t2, $p3: *mut $t3) -> $return_type {
    			unsafe {
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn( $t0, $t1, *mut $t2, *mut $t3 ) -> $return_type>(self.glfps.$name.f)( $p0, $p1, $p2, $p3 )
    			}
    		}
    	};
    	($return_type:ident $name:ident( $t0:ident $p0:ident, $t1:ident $p1:ident, const $t2:ident **$p2:ident, const $t3:ident *$p3:ident )) => {
    		#[allow(non_snake_case)]
    		#[allow(dead_code)]
    		pub /*unsafe*/ fn $name(&self, $p0: $t0, $p1: $t1, $p2: *const *const $t2, $p3: *const $t3) -> $return_type {
    			unsafe {
    				// dbg!(self.glfps.$name.f);
    				// dbg!($p2);
    				core::mem::transmute::<*const core::ffi::c_void, extern "system" fn( $t0, $t1, *const *const $t2, *const $t3 ) -> $return_type>(self.glfps.$name.f)( $p0, $p1, $p2, $p3 )
    			}
    		}
    	};
    	/*
    	(return_type:$ty name:$ident) => {

    	};
    	*/
    	($e:expr) => {
    		// unmatched $e
    		//println!("Unmatched {:?}", $e );
    		$e
    	};
    }

pub(crate) use create_gl_wrapper;

struct GlWrapperMacro {
    glfps: Glfps,
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
struct Glfps {
    glGetError: GlFunctionPointer,
    glDisable: GlFunctionPointer,
    glGenVertexArrays: GlFunctionPointer,
    glBindVertexArray: GlFunctionPointer,
    glGetShaderiv: GlFunctionPointer,
}

impl GlWrapperMacro {
    fn fuu() {}

    //create_gl_wrapper!(GLenum glGetError());
    //create_gl_wrapper!(u8);

    create_gl_wrapper!(GLenum glGetError( void ) );
    create_gl_wrapper!(void glDisable( GLenum cap ));
    create_gl_wrapper!(void glGenVertexArrays(GLsizei n, GLuint *arrays));
    create_gl_wrapper!(void glBindVertexArray(GLuint array));
    create_gl_wrapper!(void glGetShaderiv(GLuint shader, GLenum pname, GLint *params));
    /*
    create_gl_wrapper!(u8 blah( void ) );
    */
    //create_gl_wrapper!(GLuint glCreateShader);
    //create_gl_wrapper!(GLuint glCreateShader(GLenum shaderType));

    //pub fn gl_create_shader(&self,)
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_gl_wrapper_works() {
        create_gl_wrapper!(GLuint);
    }
}
