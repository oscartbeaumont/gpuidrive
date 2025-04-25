use gpui::*;

use crate::{
    components::{DataTable, PathBar},
    state::State,
};

actions!(example, [CloseWindow]);

pub struct MainWindow {
    state: Entity<State>,
    path_bar: Entity<PathBar>,
    data_table: Entity<DataTable>,
    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn init(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        let state = cx.new(|cx| State::init());
        Self {
            path_bar: cx.new(|cx| PathBar::init(cx, state.clone())),
            data_table: cx.new(|cx| DataTable::new(state.clone())),
            state,
            focus_handle,
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .on_action(|_: &CloseWindow, window, _| window.remove_window())
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .child(self.path_bar.clone())
            // .child(todo)
            .child(self.data_table.clone())
    }
}
