use gpui::*;

use crate::{
    components::{DataTable, PathBar, QuickPreview, open_node},
    state::State,
};

actions!(example, [CloseWindow]);

pub struct MainWindow {
    state: Entity<State>,
    path_bar: Entity<PathBar>,
    data_table: Entity<DataTable>,
    quick_preview: Entity<QuickPreview>,
    focus: FocusHandle,
}

impl MainWindow {
    pub fn init(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let focus = cx.focus_handle();
        focus.focus(window);

        let state = cx.new(|_| State::init());
        Self {
            path_bar: cx.new(|cx| PathBar::init(cx, state.clone())),
            data_table: cx.new(|cx| DataTable::new(state.clone())),
            quick_preview: cx.new(|cx| {
                cx.observe(&state, |quick_preview: &mut QuickPreview, state, cx| {
                    let state = state.read(cx);
                    quick_preview.set(state.selected().and_then(|s| state.nodes().get(s)).cloned());
                })
                .detach();

                QuickPreview::init()
            }),
            state,
            focus,
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            // TODO: move this onto the data view
            .on_key_down({
                let state = self.state.clone();
                let preview = self.quick_preview.clone();

                move |event, _, cx| {
                    let modifier =
                        event.keystroke.modifiers.platform || event.keystroke.modifiers.shift; // TODO: Make this better

                    match &*event.keystroke.key {
                        "[" if modifier => {
                            state.update(cx, |state, cx| state.go_back(cx));
                        }
                        "]" if modifier => {
                            state.update(cx, |state, cx| state.go_forward(cx));
                        }
                        "up" if modifier => {
                            state.update(cx, |state, cx| state.go_up(cx));
                        }
                        "down" if modifier => {
                            let s = state.read(cx);
                            if let Some(selection) = s.selected() {
                                let node = s.nodes().get(selection).unwrap().clone();

                                open_node(&state, cx, &node, false);
                            }
                        }
                        // TODO: How to navigate forward in history stack with keyboard????
                        "o" if modifier => {
                            let s = state.read(cx);
                            if let Some(selection) = s.selected() {
                                let node = s.nodes().get(selection).unwrap().clone();

                                open_node(&state, cx, &node, true);
                            }
                        }
                        "up" => {
                            state.update(cx, |state, cx| state.back_selected(cx));
                        }
                        "down" => {
                            state.update(cx, |state, cx| state.next_selected(cx));
                        }
                        "escape" => {
                            state.update(cx, |state, cx| state.clear_selection(cx));
                        }
                        "enter" => {
                            // TODO: Implement rename
                        }
                        "space" => {
                            preview.update(cx, |s, cx| {
                                s.toggle();
                                cx.notify();
                            });
                        }
                        _ => {}
                    }
                }
            })
            .font_family(".SystemUIFont")
            .on_action(|_: &CloseWindow, window, _| window.remove_window())
            .track_focus(&self.focus)
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
                            .child(self.quick_preview.clone()),
                    ),
            )
            .child(self.quick_preview.clone())
    }
}
