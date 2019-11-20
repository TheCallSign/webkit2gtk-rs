// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use WebView;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_sys;

glib_wrapper! {
    pub struct FindController(Object<webkit2_sys::WebKitFindController, webkit2_sys::WebKitFindControllerClass, FindControllerClass>);

    match fn {
        get_type => || webkit2_sys::webkit_find_controller_get_type(),
    }
}

pub const NONE_FIND_CONTROLLER: Option<&FindController> = None;

pub trait FindControllerExt: 'static {
    fn count_matches(&self, search_text: &str, find_options: u32, max_match_count: u32);

    fn get_max_match_count(&self) -> u32;

    fn get_options(&self) -> u32;

    fn get_search_text(&self) -> Option<GString>;

    fn get_web_view(&self) -> Option<WebView>;

    fn search(&self, search_text: &str, find_options: u32, max_match_count: u32);

    fn search_finish(&self);

    fn search_next(&self);

    fn search_previous(&self);

    fn get_property_text(&self) -> Option<GString>;

    fn connect_counted_matches<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_failed_to_find_text<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_found_text<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_match_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FindController>> FindControllerExt for O {
    fn count_matches(&self, search_text: &str, find_options: u32, max_match_count: u32) {
        unsafe {
            webkit2_sys::webkit_find_controller_count_matches(self.as_ref().to_glib_none().0, search_text.to_glib_none().0, find_options, max_match_count);
        }
    }

    fn get_max_match_count(&self) -> u32 {
        unsafe {
            webkit2_sys::webkit_find_controller_get_max_match_count(self.as_ref().to_glib_none().0)
        }
    }

    fn get_options(&self) -> u32 {
        unsafe {
            webkit2_sys::webkit_find_controller_get_options(self.as_ref().to_glib_none().0)
        }
    }

    fn get_search_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_find_controller_get_search_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_web_view(&self) -> Option<WebView> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_find_controller_get_web_view(self.as_ref().to_glib_none().0))
        }
    }

    fn search(&self, search_text: &str, find_options: u32, max_match_count: u32) {
        unsafe {
            webkit2_sys::webkit_find_controller_search(self.as_ref().to_glib_none().0, search_text.to_glib_none().0, find_options, max_match_count);
        }
    }

    fn search_finish(&self) {
        unsafe {
            webkit2_sys::webkit_find_controller_search_finish(self.as_ref().to_glib_none().0);
        }
    }

    fn search_next(&self) {
        unsafe {
            webkit2_sys::webkit_find_controller_search_next(self.as_ref().to_glib_none().0);
        }
    }

    fn search_previous(&self) {
        unsafe {
            webkit2_sys::webkit_find_controller_search_previous(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_text(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"text\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().ok().flatten()
        }
    }

    fn connect_counted_matches<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"counted-matches\0".as_ptr() as *const _,
                Some(transmute(counted_matches_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_failed_to_find_text<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"failed-to-find-text\0".as_ptr() as *const _,
                Some(transmute(failed_to_find_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_found_text<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"found-text\0".as_ptr() as *const _,
                Some(transmute(found_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_max_match_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max-match-count\0".as_ptr() as *const _,
                Some(transmute(notify_max_match_count_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::options\0".as_ptr() as *const _,
                Some(transmute(notify_options_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn counted_matches_trampoline<P, F: Fn(&P, u32) + 'static>(this: *mut webkit2_sys::WebKitFindController, match_count: libc::c_uint, f: glib_sys::gpointer)
where P: IsA<FindController> {
    let f: &F = &*(f as *const F);
    f(&FindController::from_glib_borrow(this).unsafe_cast(), match_count)
}

unsafe extern "C" fn failed_to_find_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitFindController, f: glib_sys::gpointer)
where P: IsA<FindController> {
    let f: &F = &*(f as *const F);
    f(&FindController::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn found_text_trampoline<P, F: Fn(&P, u32) + 'static>(this: *mut webkit2_sys::WebKitFindController, match_count: libc::c_uint, f: glib_sys::gpointer)
where P: IsA<FindController> {
    let f: &F = &*(f as *const F);
    f(&FindController::from_glib_borrow(this).unsafe_cast(), match_count)
}

unsafe extern "C" fn notify_max_match_count_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitFindController, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<FindController> {
    let f: &F = &*(f as *const F);
    f(&FindController::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_options_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitFindController, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<FindController> {
    let f: &F = &*(f as *const F);
    f(&FindController::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitFindController, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<FindController> {
    let f: &F = &*(f as *const F);
    f(&FindController::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FindController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FindController")
    }
}
