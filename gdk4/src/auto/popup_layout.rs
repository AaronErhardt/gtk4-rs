// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AnchorHints, Gravity, Rectangle};
use glib::translate::*;
#[cfg(any(feature = "v4_2", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct PopupLayout(Shared<ffi::GdkPopupLayout>);

    match fn {
        ref => |ptr| ffi::gdk_popup_layout_ref(ptr),
        unref => |ptr| ffi::gdk_popup_layout_unref(ptr),
        type_ => || ffi::gdk_popup_layout_get_type(),
    }
}

impl PopupLayout {
    #[doc(alias = "gdk_popup_layout_new")]
    pub fn new(
        anchor_rect: &Rectangle,
        rect_anchor: Gravity,
        surface_anchor: Gravity,
    ) -> PopupLayout {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_popup_layout_new(
                anchor_rect.to_glib_none().0,
                rect_anchor.into_glib(),
                surface_anchor.into_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_popup_layout_copy")]
    #[must_use]
    pub fn copy(&self) -> PopupLayout {
        unsafe { from_glib_full(ffi::gdk_popup_layout_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_popup_layout_equal")]
    fn equal(&self, other: &PopupLayout) -> bool {
        unsafe {
            from_glib(ffi::gdk_popup_layout_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_popup_layout_get_anchor_hints")]
    #[doc(alias = "get_anchor_hints")]
    pub fn anchor_hints(&self) -> AnchorHints {
        unsafe {
            from_glib(ffi::gdk_popup_layout_get_anchor_hints(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_popup_layout_get_anchor_rect")]
    #[doc(alias = "get_anchor_rect")]
    pub fn anchor_rect(&self) -> Rectangle {
        unsafe { from_glib_none(ffi::gdk_popup_layout_get_anchor_rect(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_popup_layout_get_rect_anchor")]
    #[doc(alias = "get_rect_anchor")]
    pub fn rect_anchor(&self) -> Gravity {
        unsafe { from_glib(ffi::gdk_popup_layout_get_rect_anchor(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
    #[doc(alias = "gdk_popup_layout_get_shadow_width")]
    #[doc(alias = "get_shadow_width")]
    pub fn shadow_width(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut left = mem::MaybeUninit::uninit();
            let mut right = mem::MaybeUninit::uninit();
            let mut top = mem::MaybeUninit::uninit();
            let mut bottom = mem::MaybeUninit::uninit();
            ffi::gdk_popup_layout_get_shadow_width(
                self.to_glib_none().0,
                left.as_mut_ptr(),
                right.as_mut_ptr(),
                top.as_mut_ptr(),
                bottom.as_mut_ptr(),
            );
            (
                left.assume_init(),
                right.assume_init(),
                top.assume_init(),
                bottom.assume_init(),
            )
        }
    }

    #[doc(alias = "gdk_popup_layout_get_surface_anchor")]
    #[doc(alias = "get_surface_anchor")]
    pub fn surface_anchor(&self) -> Gravity {
        unsafe {
            from_glib(ffi::gdk_popup_layout_get_surface_anchor(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_popup_layout_set_anchor_hints")]
    pub fn set_anchor_hints(&self, anchor_hints: AnchorHints) {
        unsafe {
            ffi::gdk_popup_layout_set_anchor_hints(self.to_glib_none().0, anchor_hints.into_glib());
        }
    }

    #[doc(alias = "gdk_popup_layout_set_anchor_rect")]
    pub fn set_anchor_rect(&self, anchor_rect: &Rectangle) {
        unsafe {
            ffi::gdk_popup_layout_set_anchor_rect(
                self.to_glib_none().0,
                anchor_rect.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_popup_layout_set_offset")]
    pub fn set_offset(&self, dx: i32, dy: i32) {
        unsafe {
            ffi::gdk_popup_layout_set_offset(self.to_glib_none().0, dx, dy);
        }
    }

    #[doc(alias = "gdk_popup_layout_set_rect_anchor")]
    pub fn set_rect_anchor(&self, anchor: Gravity) {
        unsafe {
            ffi::gdk_popup_layout_set_rect_anchor(self.to_glib_none().0, anchor.into_glib());
        }
    }

    #[cfg(any(feature = "v4_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
    #[doc(alias = "gdk_popup_layout_set_shadow_width")]
    pub fn set_shadow_width(&self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe {
            ffi::gdk_popup_layout_set_shadow_width(self.to_glib_none().0, left, right, top, bottom);
        }
    }

    #[doc(alias = "gdk_popup_layout_set_surface_anchor")]
    pub fn set_surface_anchor(&self, anchor: Gravity) {
        unsafe {
            ffi::gdk_popup_layout_set_surface_anchor(self.to_glib_none().0, anchor.into_glib());
        }
    }
}

impl PartialEq for PopupLayout {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for PopupLayout {}
