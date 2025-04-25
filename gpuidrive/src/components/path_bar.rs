use std::path::PathBuf;

use gpui::*;

use crate::{input::TextInput, state::State};

pub struct PathBar {
    state: Entity<State>,
    text_input: Entity<TextInput>,
}

impl PathBar {
    pub fn init(cx: &mut Context<Self>, state: Entity<State>) -> Self {
        let text_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Type here...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
        });

        cx.subscribe(&text_input, {
            let state = state.clone();

            move |subscriber, _emitter, event, cx| {
                println!("ON CHANGE {:?}", &event.0);

                state.update(cx, |state, cx| {
                    // TODO: This won't handle non-utf8 paths
                    state.set_path(PathBuf::from(event.0.to_string()));
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
            .text_color(rgb(0x0))
            .child(self.text_input.clone())
            .child(
                self.state
                    .read(cx)
                    .path()
                    // TODO: Don't do on every render
                    .to_string_lossy()
                    .to_string(),
            )
    }
}
