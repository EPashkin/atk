// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Image(Object<ffi::AtkImage, ffi::AtkImageIface>);

    match fn {
        get_type => || ffi::atk_image_get_type(),
    }
}

pub trait ImageExt {
    fn get_image_description(&self) -> Option<String>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_image_locale(&self) -> Option<String>;

    //fn get_image_position(&self, coord_type: /*Ignored*/CoordType) -> (i32, i32);

    fn get_image_size(&self) -> (i32, i32);

    fn set_image_description(&self, description: &str) -> bool;
}

impl<O: IsA<Image>> ImageExt for O {
    fn get_image_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_image_get_image_description(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_image_locale(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_image_get_image_locale(self.to_glib_none().0))
        }
    }

    //fn get_image_position(&self, coord_type: /*Ignored*/CoordType) -> (i32, i32) {
    //    unsafe { TODO: call ffi::atk_image_get_image_position() }
    //}

    fn get_image_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::atk_image_get_image_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn set_image_description(&self, description: &str) -> bool {
        unsafe {
            from_glib(ffi::atk_image_set_image_description(self.to_glib_none().0, description.to_glib_none().0))
        }
    }
}