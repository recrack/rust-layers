// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of cross-process surfaces for Linux. This uses X pixmaps.

use platform::surface::NativeSurfaceMethods;
use texturegl::Texture;

use geom::size::Size2D;
use opengles::gl2::NO_ERROR;
use opengles::gl2;
use std::cast;
use std::libc::{c_char, c_int, c_uint, c_void};
use std::libc::funcs::c95::stdlib::{malloc, free};
use std::ptr;

use egl::egl::NativeWindowType;
use egl::egl::{EGLConfig, EGLint, EGLBoolean};
use egl::egl::{EGL_NO_DISPLAY, EGL_DEFAULT_DISPLAY, EGL_NO_CONTEXT, EGL_CONTEXT_CLIENT_VERSION, EGL_NONE};
use egl::egl::{EGLDisplay, EGLSurface, EGLContext, EGL_TRUE, EGL_FALSE, EGL_WIDTH, EGL_HEIGHT};
use egl::egl::{EGL_SURFACE_TYPE, EGL_WINDOW_BIT, EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
                EGL_NONE, EGL_NO_SURFACE, EGL_BUFFER_SIZE, EGL_DEPTH_SIZE, EGL_DRAW};
use egl::egl::{GetDisplay, Initialize, CreateContext, MakeCurrent};
use egl::egl::{QuerySurface, ChooseConfig, GetError};
use egl::egl::{CreateWindowSurface, SwapBuffers, DestroyContext, GetCurrentDisplay, GetCurrentSurface, GetCurrentContext};
use egl::egl::{EGL_RED_SIZE, EGL_BLUE_SIZE, EGL_GREEN_SIZE};


/// The display and visual info. This is needed in order to upload on the painting side. This holds
/// a *strong* reference to the display and will close it when done.
///
/// FIXME(pcwalton): Mark nonsendable and noncopyable.
pub struct NativePaintingGraphicsContext{
    //To be implemented
    display:EGLDisplay
}

impl NativePaintingGraphicsContext {
    #[fixed_stack_segment]
    pub fn from_metadata(metadata: &NativeGraphicsMetadata) -> NativePaintingGraphicsContext {
        //To be implemented
        unsafe {
            NativePaintingGraphicsContext {
                display : GetCurrentDisplay(),
            }
        }
    }
}

impl Drop for NativePaintingGraphicsContext {
    #[fixed_stack_segment]
    fn drop(&mut self) {
        //To be implemented
        let display: EGLDisplay = GetDisplay(EGL_DEFAULT_DISPLAY as *c_void);
    }
}

/// The display, visual info, and framebuffer configuration. This is needed in order to bind to a
/// texture on the compositor side. This holds only a *weak* reference to the display and does not
/// close it.
///
/// FIXME(pcwalton): Unchecked weak references are bad and can violate memory safety. This is hard
/// to fix because the Display is given to us by the native windowing system, but we should fix it
/// someday.
///
/// FIXME(pcwalton): Mark nonsendable.
pub struct NativeCompositingGraphicsContext {
    //To be implemented
    display : EGLDisplay,
    surface : EGLSurface,
    context : EGLContext,
}

impl NativeCompositingGraphicsContext {
    pub fn new() -> NativeCompositingGraphicsContext {
        //To be implemented

        NativeCompositingGraphicsContext {
            display : GetCurrentDisplay(),
            surface : GetCurrentSurface(EGL_DRAW as i32),
            context : GetCurrentContext(),
        }
    }
}



pub struct NativeGraphicsMetadata {
    dummy: bool,
}

#[deriving(Eq)]
pub enum NativeSurfaceTransientData {
    NoTransientData,
}

pub struct NativeSurface {
    //To be implemented
    /// Whether this pixmap will leak if the destructor runs. This is for debugging purposes.
    pixmap: *c_void,
    will_leak: bool,
}

impl Drop for NativeSurface {
    fn drop(&mut self) {
        if self.will_leak {
            fail!("You should have disposed of the pixmap properly with destroy()! This pixmap \
                   will leak!");
        }
    }
}

impl NativeSurface {
    #[fixed_stack_segment]
    pub fn from_eglsurface(surface: /*to be changed to egl surface type*/*c_void) -> NativeSurface {
        NativeSurface {
            pixmap: surface,
            will_leak: true,
        }
    }
}

impl NativeSurfaceMethods for NativeSurface {
    #[fixed_stack_segment]
    fn new(native_context: &NativePaintingGraphicsContext, size: Size2D<i32>, stride: i32)
           -> NativeSurface {
               //To be implemented
        let mut len = size.width * size.height * (stride/size.width);
        unsafe {
             let mut data = malloc(len as u32);
               NativeSurface {
                   pixmap: cast::transmute(data),
                   will_leak: false,
               }
        }
    }

    /// This may only be called on the compositor side.
    #[fixed_stack_segment]
    fn bind_to_texture(&self,
                       native_context: &NativeCompositingGraphicsContext,
                       texture: &Texture,
                       _size: Size2D<int>) {
        //To be implemented
        // texture bind
        // glTexture2D function
    }

    /// This may only be called on the painting side.
    #[fixed_stack_segment]
    fn upload(&self, graphics_context: &NativePaintingGraphicsContext, data: &[u8]) {
        //To be implemented
    }

    fn get_id(&self) -> int {
        //To be implemented
        self.pixmap as int
    }

#[fixed_stack_segment]
    fn destroy(&mut self, graphics_context: &NativePaintingGraphicsContext) {
        unsafe {
            //To be implemented
            free(self.pixmap);
            self.mark_wont_leak()
        }
    }

    fn mark_will_leak(&mut self) {
        self.will_leak = true
    }
    fn mark_wont_leak(&mut self) {
        self.will_leak = false
    }
}
