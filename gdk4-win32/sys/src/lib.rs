// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GdkWin32MessageFilterReturn = c_int;
pub const GDK_WIN32_MESSAGE_FILTER_CONTINUE: GdkWin32MessageFilterReturn = 0;
pub const GDK_WIN32_MESSAGE_FILTER_REMOVE: GdkWin32MessageFilterReturn = 1;

// Callbacks
pub type GdkWin32MessageFilterFunc = Option<
    unsafe extern "C" fn(
        *mut GdkWin32Display,
        gpointer,
        *mut c_int,
        gpointer,
    ) -> GdkWin32MessageFilterReturn,
>;

// Records
#[repr(C)]
pub struct _GdkWin32DisplayClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWin32DisplayClass = *mut _GdkWin32DisplayClass;

#[repr(C)]
pub struct _GdkWin32DisplayManagerClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWin32DisplayManagerClass = *mut _GdkWin32DisplayManagerClass;

#[repr(C)]
pub struct _GdkWin32DragClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWin32DragClass = *mut _GdkWin32DragClass;

#[repr(C)]
pub struct _GdkWin32GLContextClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWin32GLContextClass = *mut _GdkWin32GLContextClass;

#[repr(C)]
pub struct _GdkWin32HCursorClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWin32HCursorClass = *mut _GdkWin32HCursorClass;

#[repr(C)]
pub struct _GdkWin32MonitorClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWin32MonitorClass = *mut _GdkWin32MonitorClass;

#[repr(C)]
pub struct _GdkWin32ScreenClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWin32ScreenClass = *mut _GdkWin32ScreenClass;

#[repr(C)]
pub struct _GdkWin32SurfaceClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWin32SurfaceClass = *mut _GdkWin32SurfaceClass;

// Classes
#[repr(C)]
pub struct GdkWin32Display {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWin32Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWin32Display @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct GdkWin32DisplayManager {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWin32DisplayManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWin32DisplayManager @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct GdkWin32Drag {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWin32Drag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWin32Drag @ {self:p}")).finish()
    }
}

#[repr(C)]
pub struct GdkWin32GLContext {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWin32GLContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWin32GLContext @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct GdkWin32HCursor {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWin32HCursor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWin32HCursor @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct GdkWin32Monitor {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWin32Monitor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWin32Monitor @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct GdkWin32Screen {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWin32Screen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWin32Screen @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct GdkWin32Surface {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWin32Surface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWin32Surface @ {self:p}"))
            .finish()
    }
}

#[link(name = "gtk-4")]
extern "C" {

    //=========================================================================
    // GdkWin32Display
    //=========================================================================
    pub fn gdk_win32_display_get_type() -> GType;
    pub fn gdk_win32_display_get_primary_monitor(
        display: *mut gdk::GdkDisplay,
    ) -> *mut gdk::GdkMonitor;
    pub fn gdk_win32_display_get_wgl_version(
        display: *mut gdk::GdkDisplay,
        major: *mut c_int,
        minor: *mut c_int,
    ) -> gboolean;
    pub fn gdk_win32_display_add_filter(
        display: *mut GdkWin32Display,
        function: GdkWin32MessageFilterFunc,
        data: gpointer,
    );
    #[cfg(any(feature = "v4_4", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn gdk_win32_display_get_egl_display(display: *mut GdkWin32Display) -> gpointer;
    pub fn gdk_win32_display_get_win32hcursor(
        display: *mut GdkWin32Display,
        cursor: *mut gdk::GdkCursor,
    ) -> *mut GdkWin32HCursor;
    pub fn gdk_win32_display_remove_filter(
        display: *mut GdkWin32Display,
        function: GdkWin32MessageFilterFunc,
        data: gpointer,
    );
    pub fn gdk_win32_display_set_cursor_theme(
        display: *mut GdkWin32Display,
        name: *const c_char,
        size: c_int,
    );

    //=========================================================================
    // GdkWin32DisplayManager
    //=========================================================================
    pub fn gdk_win32_display_manager_get_type() -> GType;

    //=========================================================================
    // GdkWin32Drag
    //=========================================================================
    pub fn gdk_win32_drag_get_type() -> GType;

    //=========================================================================
    // GdkWin32GLContext
    //=========================================================================
    pub fn gdk_win32_gl_context_get_type() -> GType;

    //=========================================================================
    // GdkWin32HCursor
    //=========================================================================
    pub fn gdk_win32_hcursor_get_type() -> GType;
    pub fn gdk_win32_hcursor_new(
        display: *mut GdkWin32Display,
        handle: ssize_t,
        destroyable: gboolean,
    ) -> *mut GdkWin32HCursor;

    //=========================================================================
    // GdkWin32Monitor
    //=========================================================================
    pub fn gdk_win32_monitor_get_type() -> GType;
    pub fn gdk_win32_monitor_get_workarea(
        monitor: *mut gdk::GdkMonitor,
        workarea: *mut gdk::GdkRectangle,
    );

    //=========================================================================
    // GdkWin32Screen
    //=========================================================================
    pub fn gdk_win32_screen_get_type() -> GType;

    //=========================================================================
    // GdkWin32Surface
    //=========================================================================
    pub fn gdk_win32_surface_get_type() -> GType;
    pub fn gdk_win32_surface_get_impl_hwnd(surface: *mut gdk::GdkSurface) -> ssize_t;
    pub fn gdk_win32_surface_is_win32(surface: *mut gdk::GdkSurface) -> gboolean;
    pub fn gdk_win32_surface_lookup_for_display(
        display: *mut gdk::GdkDisplay,
        anid: ssize_t,
    ) -> *mut gdk::GdkSurface;
    pub fn gdk_win32_surface_get_handle(surface: *mut GdkWin32Surface) -> ssize_t;
    pub fn gdk_win32_surface_set_urgency_hint(surface: *mut GdkWin32Surface, urgent: gboolean);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gdk_win32_handle_table_lookup(handle: ssize_t) -> gpointer;

}
