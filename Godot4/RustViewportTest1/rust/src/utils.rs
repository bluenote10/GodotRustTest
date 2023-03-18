#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

use godot::engine::control::{GrowDirection, LayoutPreset};
use godot::engine::global::Side;
use godot::engine::Control;
use godot::prelude::*;

macro_rules! gd_add_child {
    ($base:expr, $child:expr) => {
        $base.add_child(
            $child.upcast(),
            false,
            godot::engine::node::InternalMode::INTERNAL_MODE_DISABLED,
        );
    };
}

pub(crate) use gd_add_child;

macro_rules! gd_set_anchor_left {
    ($control:expr, $value:expr) => {
        $control.set_anchor(godot::engine::global::Side::SIDE_LEFT, $value, false, true);
    };
}
macro_rules! gd_set_anchor_right {
    ($control:expr, $value:expr) => {
        $control.set_anchor(godot::engine::global::Side::SIDE_RIGHT, $value, false, true);
    };
}
macro_rules! gd_set_anchor_top {
    ($control:expr, $value:expr) => {
        $control.set_anchor(godot::engine::global::Side::SIDE_TOP, $value, false, true);
    };
}
macro_rules! gd_set_anchor_bottom {
    ($control:expr, $value:expr) => {
        $control.set_anchor(
            godot::engine::global::Side::SIDE_BOTTOM,
            $value,
            false,
            true,
        );
    };
}

pub(crate) use gd_set_anchor_bottom;
pub(crate) use gd_set_anchor_left;
pub(crate) use gd_set_anchor_right;
pub(crate) use gd_set_anchor_top;

pub fn set_full_rect(control: &mut Gd<Control>) {
    control.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT, false);
    control.set_anchor(Side::SIDE_RIGHT, 1.0, false, true);
    control.set_anchor(Side::SIDE_BOTTOM, 1.0, false, true);
    control.set_h_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
    control.set_v_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
}

pub fn set_full_rect_generic<C>(control: &mut Gd<C>)
where
    C: GodotClass + Inherits<Control>,
{
    let mut control = control.share().upcast();
    control.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT, false);
    control.set_anchor(Side::SIDE_RIGHT, 1.0, false, true);
    control.set_anchor(Side::SIDE_BOTTOM, 1.0, false, true);
    control.set_h_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
    control.set_v_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
}

/*
fn test_call_from_gd_subclass(mut label: Gd<Label>) {
    // This works, because Label implements Inherits<Control>
    do_something_with_control(&mut label);
}

fn test_call_from_base_subclass(mut label: Base<Label>) {
    // This should work as well (is `deref_mut` the preferred way to go from Base to Gd?)
    do_something_with_control(label.deref_mut());
}

fn test_call_from_gd_self_type(mut control: Gd<Control>) {
    // Doesn't work, Control does not implement Inherits<Control>
    do_something_with_control(&mut control);
}

fn test_call_from_base_self_type(mut label: Base<Control>) {
    // Doesn't work, Control does not implement Inherits<Control>
    do_something_with_control(label.deref_mut());
}

pub fn do_something_with_control<C>(control: &mut Gd<C>)
where
    C: GodotClass + Inherits<Control>,
{
    let mut control = control.share().upcast();
    control.grab_focus()
}
*/