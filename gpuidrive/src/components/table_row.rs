use std::rc::Rc;

use gpui::{prelude::FluentBuilder, *};
use human_bytes::human_bytes;
use opener::open;

use crate::state::{Node, NodeKind, State};

#[derive(IntoElement)]
pub struct TableRow {
    ix: usize,
    node: Rc<Node>,
    selected: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TableRow {
    pub fn new(ix: usize, node: Rc<Node>, selected: bool) -> Self {
        Self {
            ix,
            node,
            selected,
            on_click: None,
        }
    }

    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    fn render_cell(
        &self,
        key: &str,
        width: DefiniteLength,
        cx: &mut App,
    ) -> impl IntoElement + use<> {
        div()
            .whitespace_nowrap()
            .truncate()
            .w(width)
            .px_1()
            .child(match key {
                "name" => div().child(self.node.name.to_string_lossy().to_string()),
                "kind" => div().child(format!("{:?}", self.node.kind)),
                "size" => div().child(human_bytes(self.node.size as f64)), // TODO: This cast is bad
                "created" => div().child(self.node.created.format("%B %d, %Y").to_string()),
                "modified" => div().child(self.node.modified.format("%B %d, %Y").to_string()),
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
        div()
            .id(self.ix) // TODO: Should this be scoped to `TableRow` component instance??
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(rgb(0xE0E0E0))
            .bg(if self.selected {
                rgb(0xbababa)
            } else if self.ix % 2 == 0 {
                rgb(0xFFFFFF)
            } else {
                rgb(0xFAFAFA)
            })
            .py_0p5()
            .px_2()
            .w_full()
            .children(FIELDS.map(|(key, width)| self.render_cell(key, relative(width), cx)))
            .when_some(self.on_click, move |this, on_click| {
                this.cursor_pointer().on_click(on_click)
            })
    }
}

pub fn open_node(state: &Entity<State>, cx: &mut App, node: &Node, force: bool) {
    match node.kind {
        NodeKind::Directory if !force => {
            let path = node.path.clone();

            state.update(cx, move |state: &mut State, cx| state.set_path(cx, path));
        }
        NodeKind::File | NodeKind::Directory => {
            open(node.path.clone()).unwrap();
        }
        NodeKind::Unknown => {}
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
