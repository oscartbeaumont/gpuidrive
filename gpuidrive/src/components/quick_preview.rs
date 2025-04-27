use std::{rc::Rc, time::Duration};

use gpui::*;
use opener::open;

use crate::state::{Node, NodeKind, State};

pub struct QuickPreview {
    node: Option<Rc<Node>>,
    toggle: bool,
}

impl QuickPreview {
    pub fn init() -> Self {
        Self {
            node: None,
            toggle: false,
        }
    }

    pub fn set(&mut self, node: Option<Rc<Node>>) {
        self.node = node;
    }

    pub fn toggle(&mut self) {
        self.toggle = !self.toggle;
    }
}

impl Render for QuickPreview {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        if !self.toggle {
            return div().into_any();
        }
        let Some(node) = self.node.clone() else {
            self.toggle = false;
            return div().into_any();
        };

        div()
            .with_animation(
                "loading-bg",
                Animation::new(Duration::from_millis(100)),
                move |this, delta| {
                    this.absolute()
                        .inset_0()
                        .size_full()
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .size_full()
                                .bg(black().opacity(delta)),
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
                                        .bg(gpui::black().alpha(delta * 0.2))
                                        .child(
                                            div()
                                                .flex()
                                                .justify_center()
                                                .text_color(white())
                                                .bg(rgb(0x696969))
                                                .child(node.name.to_string_lossy().to_string()),
                                        )
                                        .child(div().opacity(delta).child(match node.kind {
                                            NodeKind::File => match node.path.extension() {
                                                // TODO: Wayyy smarter matching
                                                // TODO: Support more file types (video, audio, 3D model)
                                                // TODO: Use gpui::Img::extensions() for image detection
                                                Some(s) if s == "png" => image_preview(&node),
                                                Some(s) if s == "txt" => text_preview(&node),
                                                _ => {
                                                    placeholder_preview("Unknown File Type!".into())
                                                }
                                            },
                                            NodeKind::Directory => placeholder_preview(
                                                node.name.to_str().unwrap().to_string(),
                                            ),
                                            NodeKind::Unknown => {
                                                placeholder_preview("Unknown File Type!".into())
                                            }
                                        })),
                                ),
                        )
                },
            )
            .into_any()
    }
}

fn text_preview(node: &Node) -> Div {
    // TODO: Handle non-UTF8 file content
    // TODO: We should probs be caching this between renders
    let content = std::fs::read_to_string(&node.path).unwrap();

    // ImageAssetLoader

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
