use std::path::PathBuf;

use gpui::*;

use crate::{
    components::TextInput,
    state::{PathChange, State},
};

use super::{button, button2};

pub struct PathBar {
    state: Entity<State>,
    text_input: Entity<TextInput>,
}

impl PathBar {
    pub fn init(cx: &mut Context<Self>, state: Entity<State>) -> Self {
        let text_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: SharedString::new(state.read(cx).path().to_str().unwrap().to_string()), // TODO: Utf-8 strings
            placeholder: "Type here...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
        });

        // TODO: this is a 2 way data binding which is cringe

        cx.subscribe(&text_input, {
            let state = state.clone();

            move |subscriber, _emitter, event, cx| {
                state.update(cx, |state, cx| {
                    // TODO: This won't handle non-utf8 paths
                    state.set_path(cx, PathBuf::from(event.0.to_string()));
                });
            }
        })
        .detach();

        cx.subscribe(&state, {
            let state = state.clone();
            let text_input = text_input.clone();

            move |subscriber, _emitter, event: &PathChange, cx| {
                let path = state.read(cx).path().to_str().unwrap().to_string();
                text_input.update(cx, |text_input, cx| {
                    text_input.content = SharedString::new(path);
                });
            }
        })
        .detach();

        Self { state, text_input }
    }
}

impl Render for PathBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x0))
            .child(div().w_full().child(self.text_input.clone()))
            .child(button2("Up", !self.state.read(cx).can_go_up(), {
                let state = self.state.clone();
                move |_, cx| {
                    state.update(cx, |state, cx| state.go_up(cx));
                }
            }))
            .child(button2("Back", !self.state.read(cx).can_go_back(), {
                let state = self.state.clone();
                move |_, cx| {
                    state.update(cx, |state, cx| state.go_back(cx));
                }
            }))
            .child(button2("Forward", !self.state.read(cx).can_go_forward(), {
                let state = self.state.clone();
                move |_, cx| {
                    state.update(cx, |state, cx| state.go_forward(cx));
                }
            }))
    }
}
