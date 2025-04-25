use std::path::PathBuf;

use gpui::*;

use crate::{data_table::DataTable, input::TextInput};

actions!(example, [CloseWindow]);

pub struct MainWindow {
    pub path: PathBuf,
    pub text_input: Entity<TextInput>,
    pub focus_handle: FocusHandle,
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let table = cx.new(|cx| {
            let mut table = DataTable::new();
            table.generate();
            table
        });

        div()
            .on_action(|_: &CloseWindow, window, _| window.remove_window())
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .text_color(rgb(0xffffff))
            .child(self.text_input.clone())
            .child(table)
    }
}

fn button(text: &str, on_click: impl Fn(&mut Window, &mut App) + 'static) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_2()
        .bg(rgb(0xf7f7f7))
        .active(|this| this.opacity(0.85))
        .border_1()
        .border_color(rgb(0xe0e0e0))
        .rounded_sm()
        .cursor_pointer()
        .child(text.to_string())
        .on_click(move |_, window, cx| on_click(window, cx))
}
