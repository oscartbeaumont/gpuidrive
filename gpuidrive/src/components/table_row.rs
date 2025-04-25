use gpui::*;
use human_bytes::human_bytes;
use opener::open;

use crate::state::{NodeKind, State};

#[derive(IntoElement)]
pub struct TableRow {
    ix: usize,
    state: Entity<State>,
}

impl TableRow {
    pub fn new(ix: usize, state: Entity<State>) -> Self {
        Self { ix, state }
    }

    fn render_cell(
        &self,
        key: &str,
        width: DefiniteLength,
        cx: &mut App,
    ) -> impl IntoElement + use<> {
        // TODO: Don't do this on a per-cell basis
        let this = self.state.read(cx).nodes().get(self.ix).unwrap(); // TODO

        div()
            .whitespace_nowrap()
            .truncate()
            .w(width)
            .px_1()
            .child(match key {
                "name" => div().child(this.name.to_string_lossy().to_string()),
                "kind" => div().child(format!("{:?}", this.kind)),
                "size" => div().child(human_bytes(this.size as f64)), // TODO: This cast is bad
                "created" => div().child(this.created.format("%B %d, %Y").to_string()),
                "modified" => div().child(this.modified.format("%B %d, %Y").to_string()),
                _ => div().child("--"),
            })
    }
}

const FIELDS: [(&str, f32); 5] = [
    ("name", 0.7),
    ("kind", 0.07),
    ("size", 0.05),
    ("created", 0.09),
    ("modified", 0.09),
];

impl RenderOnce for TableRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        // let this = self.state.read(cx).nodes().get(self.ix).unwrap(); // TODO

        div()
            .id(self.ix) // TODO: Should this be scoped to `TableRow` component instance??
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(rgb(0xE0E0E0))
            .bg(if self.ix % 2 == 0 {
                rgb(0xFFFFFF)
            } else {
                rgb(0xFAFAFA)
            })
            .py_0p5()
            .px_2()
            .w_full()
            .children(FIELDS.map(|(key, width)| self.render_cell(key, relative(width), cx)))
            .on_click(move |event, window, cx| {
                let node = self.state.read(cx).nodes().get(self.ix).unwrap();

                match node.kind {
                    NodeKind::Directory
                        if !(event.down.modifiers.platform || event.down.modifiers.control) =>
                    {
                        let path = node.path.clone();

                        self.state
                            .update(cx, move |state: &mut State, cx| state.set_path(cx, path));
                    }
                    NodeKind::File | NodeKind::Directory => {
                        open(node.path.clone()).unwrap();
                    }
                    NodeKind::Unknown => {}
                }
            })
    }
}

pub fn render_titles() -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .w_full()
        .overflow_hidden()
        .border_b_1()
        .border_color(rgb(0xE0E0E0))
        .text_color(rgb(0x555555))
        .bg(rgb(0xF0F0F0))
        .py_1()
        .px_2()
        .text_xs()
        .children(FIELDS.map(|(key, width)| {
            div()
                .whitespace_nowrap()
                .flex_shrink_0()
                .truncate()
                .px_1()
                .w(relative(width))
                .child(key.replace("_", " ").to_uppercase())
        }))
}
