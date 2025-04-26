use gpui::*;
use opener::open;

use crate::state::{Node, NodeKind, State};

pub struct QuickPreview {
    state: Entity<State>,
    visible: bool,
}

impl QuickPreview {
    pub fn init(state: Entity<State>) -> Self {
        Self {
            state,
            visible: false,
        }
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.state.read(cx).selected().is_some() {
            self.visible = !self.visible;
            cx.notify();
        }
    }
}

impl Render for QuickPreview {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        if !self.visible {
            return div();
        }
        let state = self.state.read(cx);
        let Some(selected) = state.selected() else {
            self.visible = false;
            return div();
        };
        let node = state.nodes().get(selected).unwrap();

        div()
            .absolute()
            .inset_0()
            .size_full()
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .size_full()
                    .bg(gpui::black())
                    .opacity(0.5),
            )
            .child(
                div()
                    .flex()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .w(relative(0.5))
                            .h(relative(0.5))
                            .bg(gpui::black().alpha(0.2))
                            .child(
                                div()
                                    .flex()
                                    .justify_center()
                                    .text_color(white())
                                    .bg(rgb(0x696969))
                                    .child(node.name.to_string_lossy().to_string()),
                            )
                            .child(match node.kind {
                                NodeKind::File => match node.path.extension() {
                                    // TODO: Wayyy smarter matching
                                    // TODO: Support more file types (video, audio, 3D model)
                                    Some(s) if s == "png" => image_preview(&node),
                                    Some(s) if s == "txt" => text_preview(&node),
                                    _ => placeholder_preview("Unknown File Type!".into()),
                                },
                                NodeKind::Directory => {
                                    placeholder_preview(node.name.to_str().unwrap().to_string())
                                }
                                NodeKind::Unknown => {
                                    placeholder_preview("Unknown File Type!".into())
                                }
                            }),
                    ),
            )
    }
}

fn text_preview(node: &Node) -> Div {
    // TODO: Handle non-UTF8 file content
    // TODO: We should probs be caching this between renders
    let content = std::fs::read_to_string(&node.path).unwrap();

    div()
        .flex()
        .justify_center()
        .items_center()
        .size_full()
        .bg(red())
        .text_color(white())
        .child(content)
}

fn image_preview(node: &Node) -> Div {
    div().bg(white()).child(
        img(node.path.clone())
            .id("quick-preview") // TODO: Scope this ID to the asset and make it not do lossy stuff
            .border_1()
            .size_full()
            .border_color(red())
            // TODO:
            // .with_fallback(|| Self::fallback_element().into_any_element())
            // .with_loading(|| Self::loading_element().into_any_element())
            .on_click({
                let path = node.path.clone();
                move |_, _, cx| {
                    open(&path).unwrap();
                }
            }),
    )
}

fn placeholder_preview(s: String) -> Div {
    div()
        .flex()
        .justify_center()
        .items_center()
        .size_full()
        .bg(red())
        .text_color(white())
        .child(s)
}
