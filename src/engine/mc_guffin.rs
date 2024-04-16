use color_eyre::eyre::eyre;
use color_eyre::Result;
use core::ffi::c_short;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ffi::CStr;
use core::mem::transmute;

#[derive(Debug)]
pub struct McGuffin {
    gl_rects: *const c_void,
    //gl_rects: extern "system" fn(i16, i16, i16, i16) -> c_void,
    gl_get_error: *const c_void,
}

impl Default for McGuffin {
    fn default() -> Self {
        Self {
            gl_rects: core::ptr::null(),
            gl_get_error: core::ptr::null(),
        }
    }
}

impl McGuffin {
    fn call_gl_rects(&self, x1: i16, y1: i16, x2: i16, y2: i16) {
        unsafe {
            let fn_p = transmute::<
                *const c_void,
                extern "system" fn(c_short, c_short, c_short, c_short) -> c_void,
            >(self.gl_rects);
            fn_p(x1, y1, x2, y2);
        };
    }

    fn call_gl_get_error(&self) -> c_uint {
        unsafe {
            let fn_p =
                transmute::<*const c_void, extern "system" fn() -> c_uint>(self.gl_get_error);
            fn_p()
        }
    }

    fn load_function(
        get_proc_address: &dyn Fn(&CStr) -> *const c_void,
        name: &CStr,
    ) -> Result<*const c_void> {
        let addr = get_proc_address(name);
        if addr == core::ptr::null() {
            Err(eyre!("Failed loading {name:?}").into())
        } else {
            Ok(addr)
        }
    }
    pub fn setup(&mut self, get_proc_address: &dyn Fn(&CStr) -> *const c_void) -> Result<()> {
        // load the gl functions we need
        // glRects

        self.gl_rects = Self::load_function(
            get_proc_address,
            CStr::from_bytes_with_nul(b"glRects\0").unwrap(),
        )?;
        self.gl_get_error = Self::load_function(
            get_proc_address,
            CStr::from_bytes_with_nul(b"glGetError\0").unwrap(),
        )?;

        // create the program (vertex + fragment)
        Ok(())
        //Err( eyre!("test") )
    }

    pub fn update(&mut self) -> Result<()> {
        // bind the program

        // pass in uniforms
        // e.g. current time

        // render something
        // -> e.g. a fullscreen (or rather full viewport) rectangle

        //glRects( -1, -1, 1, 1);
        self.call_gl_rects(-1, -1, 1, 1);

        let error = self.call_gl_get_error();
        match error {
            0 => {}
            0x0502 => {
                eprintln!("GL_INVALID_OPERATION");
            }
            e => {
                eprintln!("0x{e:04x?}");
            }
        }

        Ok(())
    }
}
