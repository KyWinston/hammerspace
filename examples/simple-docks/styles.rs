use bevy::prelude::*;
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderBackground, StyleBuilderBorderColor, StyleBuilderLayout, StyleBuilderPointerEvents};
use bevy_quill_obsidian::colors;



pub fn style_main(ss: &mut StyleBuilder) {
    ss.position(PositionType::Absolute)
        .left(0)
        .top(0)
        .bottom(0)
        .right(0)
        .border(1)
        .border_color(colors::U2)
        .display(Display::Flex)
        .pointer_events(false);
}

pub fn style_aside(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .background_color(colors::U2)
        .padding(8)
        .gap(8)
        .flex_direction(FlexDirection::Column)
        .width(200)
        .pointer_events(true);
}

pub fn wrapper_style(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_grow(1.)
        .align_self(AlignSelf::Stretch)
        .flex_direction(FlexDirection::Column);
}

pub fn style_button_row(ss: &mut StyleBuilder) {
    ss.gap(8);
}

pub fn style_button_flex(ss: &mut StyleBuilder) {
    ss.flex_grow(1.);
}

pub fn style_slider(ss: &mut StyleBuilder) {
    ss.align_self(AlignSelf::Stretch);
}

pub fn style_column_group(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .align_items(AlignItems::FlexStart)
        .gap(8);
}

pub fn style_scroll_area(ss: &mut StyleBuilder) {
    ss.flex_grow(1.0);
}