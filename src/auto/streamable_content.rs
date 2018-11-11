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
    pub struct StreamableContent(Object<ffi::AtkStreamableContent, ffi::AtkStreamableContentIface>);

    match fn {
        get_type => || ffi::atk_streamable_content_get_type(),
    }
}

pub trait StreamableContentExt {
    fn get_mime_type(&self, i: i32) -> Option<String>;

    fn get_n_mime_types(&self) -> i32;

    //fn get_stream(&self, mime_type: &str) -> /*Ignored*/Option<glib::IOChannel>;

    fn get_uri(&self, mime_type: &str) -> Option<String>;
}

impl<O: IsA<StreamableContent>> StreamableContentExt for O {
    fn get_mime_type(&self, i: i32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_streamable_content_get_mime_type(self.to_glib_none().0, i))
        }
    }

    fn get_n_mime_types(&self) -> i32 {
        unsafe {
            ffi::atk_streamable_content_get_n_mime_types(self.to_glib_none().0)
        }
    }

    //fn get_stream(&self, mime_type: &str) -> /*Ignored*/Option<glib::IOChannel> {
    //    unsafe { TODO: call ffi::atk_streamable_content_get_stream() }
    //}

    fn get_uri(&self, mime_type: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_streamable_content_get_uri(self.to_glib_none().0, mime_type.to_glib_none().0))
        }
    }
}