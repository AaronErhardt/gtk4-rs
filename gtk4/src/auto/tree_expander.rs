// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow,
    TreeListRow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkTreeExpander")]
    pub struct TreeExpander(Object<ffi::GtkTreeExpander, ffi::GtkTreeExpanderClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_tree_expander_get_type(),
    }
}

impl TreeExpander {
    #[doc(alias = "gtk_tree_expander_new")]
    pub fn new() -> TreeExpander {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_tree_expander_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`TreeExpander`] objects.
    ///
    /// This method returns an instance of [`TreeExpanderBuilder`](crate::builders::TreeExpanderBuilder) which can be used to create [`TreeExpander`] objects.
    pub fn builder() -> TreeExpanderBuilder {
        TreeExpanderBuilder::new()
    }

    #[doc(alias = "gtk_tree_expander_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_tree_expander_get_child(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_10", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_tree_expander_get_hide_expander")]
    #[doc(alias = "get_hide_expander")]
    pub fn hides_expander(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_expander_get_hide_expander(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v4_10", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_tree_expander_get_indent_for_depth")]
    #[doc(alias = "get_indent_for_depth")]
    pub fn is_indent_for_depth(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_expander_get_indent_for_depth(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v4_6", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_tree_expander_get_indent_for_icon")]
    #[doc(alias = "get_indent_for_icon")]
    pub fn is_indent_for_icon(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_expander_get_indent_for_icon(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_expander_get_item")]
    #[doc(alias = "get_item")]
    pub fn item(&self) -> Option<glib::Object> {
        unsafe { from_glib_full(ffi::gtk_tree_expander_get_item(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_expander_get_list_row")]
    #[doc(alias = "get_list_row")]
    pub fn list_row(&self) -> Option<TreeListRow> {
        unsafe { from_glib_none(ffi::gtk_tree_expander_get_list_row(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_expander_set_child")]
    pub fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_tree_expander_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v4_10", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_tree_expander_set_hide_expander")]
    pub fn set_hide_expander(&self, hide_expander: bool) {
        unsafe {
            ffi::gtk_tree_expander_set_hide_expander(
                self.to_glib_none().0,
                hide_expander.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v4_10", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_tree_expander_set_indent_for_depth")]
    pub fn set_indent_for_depth(&self, indent_for_depth: bool) {
        unsafe {
            ffi::gtk_tree_expander_set_indent_for_depth(
                self.to_glib_none().0,
                indent_for_depth.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v4_6", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_tree_expander_set_indent_for_icon")]
    pub fn set_indent_for_icon(&self, indent_for_icon: bool) {
        unsafe {
            ffi::gtk_tree_expander_set_indent_for_icon(
                self.to_glib_none().0,
                indent_for_icon.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tree_expander_set_list_row")]
    pub fn set_list_row(&self, list_row: Option<&TreeListRow>) {
        unsafe {
            ffi::gtk_tree_expander_set_list_row(self.to_glib_none().0, list_row.to_glib_none().0);
        }
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&TreeExpander) + 'static>(
            this: *mut ffi::GtkTreeExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "hide-expander")]
    pub fn connect_hide_expander_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hide_expander_trampoline<F: Fn(&TreeExpander) + 'static>(
            this: *mut ffi::GtkTreeExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hide-expander\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hide_expander_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "indent-for-depth")]
    pub fn connect_indent_for_depth_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indent_for_depth_trampoline<F: Fn(&TreeExpander) + 'static>(
            this: *mut ffi::GtkTreeExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::indent-for-depth\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_indent_for_depth_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_6", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "indent-for-icon")]
    pub fn connect_indent_for_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indent_for_icon_trampoline<F: Fn(&TreeExpander) + 'static>(
            this: *mut ffi::GtkTreeExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::indent-for-icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_indent_for_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "item")]
    pub fn connect_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_item_trampoline<F: Fn(&TreeExpander) + 'static>(
            this: *mut ffi::GtkTreeExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "list-row")]
    pub fn connect_list_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_list_row_trampoline<F: Fn(&TreeExpander) + 'static>(
            this: *mut ffi::GtkTreeExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::list-row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_list_row_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TreeExpander {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`TreeExpander`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TreeExpanderBuilder {
    builder: glib::object::ObjectBuilder<'static, TreeExpander>,
}

impl TreeExpanderBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    #[cfg(any(feature = "v4_10", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn hide_expander(self, hide_expander: bool) -> Self {
        Self {
            builder: self.builder.property("hide-expander", hide_expander),
        }
    }

    #[cfg(any(feature = "v4_10", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn indent_for_depth(self, indent_for_depth: bool) -> Self {
        Self {
            builder: self.builder.property("indent-for-depth", indent_for_depth),
        }
    }

    #[cfg(any(feature = "v4_6", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub fn indent_for_icon(self, indent_for_icon: bool) -> Self {
        Self {
            builder: self.builder.property("indent-for-icon", indent_for_icon),
        }
    }

    pub fn list_row(self, list_row: &TreeListRow) -> Self {
        Self {
            builder: self.builder.property("list-row", list_row.clone()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TreeExpander`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TreeExpander {
        self.builder.build()
    }
}

impl fmt::Display for TreeExpander {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeExpander")
    }
}
