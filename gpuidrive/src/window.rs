use gpui::*;

use crate::{
    components::{DataTable, PathBar, Preview},
    state::State,
};

actions!(example, [CloseWindow]);

pub struct MainWindow {
    state: Entity<State>,
    path_bar: Entity<PathBar>,
    data_table: Entity<DataTable>,
    preview: Entity<Preview>,
    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn init(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        let state = cx.new(|_| State::init());
        Self {
            path_bar: cx.new(|cx| PathBar::init(cx, state.clone())),
            data_table: cx.new(|cx| DataTable::new(state.clone())),
            preview: cx.new(|cx| Preview::init(state.clone())),
            state,
            focus_handle,
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .on_action(|_: &CloseWindow, window, _| window.remove_window())
            .track_focus(&self.focus_handle)
            .relative() // Makes this the positioning context for absolute children
            .size_full() // Or whatever size you need
            .child(
                div()
                    .absolute()
                    .inset_0() // This spreads the div to fill the container
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .size_full()
                            .child(self.path_bar.clone())
                            .child(self.data_table.clone())
                            .child(self.preview.clone()),
                    ),
            )
            .child(self.preview.clone())
    }
}
