use gpui::*;

use crate::{components::path_bar::PathBar, data_table::DataTable, state::State};

actions!(example, [CloseWindow]);

pub struct MainWindow {
    state: Entity<State>,
    path_bar: Entity<PathBar>,
    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn init(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        let state = cx.new(|_| State::init());

        Self {
            path_bar: cx.new(|cx| PathBar::init(cx, state.clone())),
            state,
            focus_handle,
        }
    }
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
            .child(self.path_bar.clone())
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
