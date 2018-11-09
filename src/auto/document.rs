// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_12", feature = "dox"))]
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Document(Object<ffi::AtkDocument, ffi::AtkDocumentIface>);

    match fn {
        get_type => || ffi::atk_document_get_type(),
    }
}

pub trait DocumentExt {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_attribute_value(&self, attribute_name: &str) -> Option<String>;

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //fn get_attributes(&self) -> /*Ignored*/Option<AttributeSet>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_current_page_number(&self) -> i32;

    //fn get_document(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    fn get_document_type(&self) -> Option<String>;

    #[cfg_attr(feature = "v2_7_90", deprecated)]
    fn get_locale(&self) -> Option<String>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_page_count(&self) -> i32;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_attribute_value(&self, attribute_name: &str, attribute_value: &str) -> bool;

    fn connect_load_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_load_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn connect_page_changed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Document> + IsA<glib::object::Object>> DocumentExt for O {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_attribute_value(&self, attribute_name: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_document_get_attribute_value(self.to_glib_none().0, attribute_name.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //fn get_attributes(&self) -> /*Ignored*/Option<AttributeSet> {
    //    unsafe { TODO: call ffi::atk_document_get_attributes() }
    //}

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_current_page_number(&self) -> i32 {
        unsafe {
            ffi::atk_document_get_current_page_number(self.to_glib_none().0)
        }
    }

    //fn get_document(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::atk_document_get_document() }
    //}

    fn get_document_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_document_get_document_type(self.to_glib_none().0))
        }
    }

    fn get_locale(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_document_get_locale(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_page_count(&self) -> i32 {
        unsafe {
            ffi::atk_document_get_page_count(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_attribute_value(&self, attribute_name: &str, attribute_value: &str) -> bool {
        unsafe {
            from_glib(ffi::atk_document_set_attribute_value(self.to_glib_none().0, attribute_name.to_glib_none().0, attribute_value.to_glib_none().0))
        }
    }

    fn connect_load_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "load-complete",
                transmute(load_complete_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_load_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "load-stopped",
                transmute(load_stopped_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn connect_page_changed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "page-changed",
                transmute(page_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "reload",
                transmute(reload_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn load_complete_trampoline<P>(this: *mut ffi::AtkDocument, f: glib_ffi::gpointer)
where P: IsA<Document> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Document::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn load_stopped_trampoline<P>(this: *mut ffi::AtkDocument, f: glib_ffi::gpointer)
where P: IsA<Document> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Document::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
unsafe extern "C" fn page_changed_trampoline<P>(this: *mut ffi::AtkDocument, page_number: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Document> {
    let f: &&(Fn(&P, i32) + 'static) = transmute(f);
    f(&Document::from_glib_borrow(this).downcast_unchecked(), page_number)
}

unsafe extern "C" fn reload_trampoline<P>(this: *mut ffi::AtkDocument, f: glib_ffi::gpointer)
where P: IsA<Document> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Document::from_glib_borrow(this).downcast_unchecked())
}