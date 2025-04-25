use std::{
    ops::Range,
    path::PathBuf,
    rc::Rc,
    time::{Duration, Instant},
};

use gpui::{
    App, Application, Bounds, Context, DefiniteLength, Entity, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Point, Render, SharedString, UniformListScrollHandle, Window,
    WindowBounds, WindowOptions, canvas, div, point, prelude::*, px, relative, rgb, size,
    uniform_list,
};

use crate::state::{Node, State};

use super::{TableRow, render_titles};

const TOTAL_ITEMS: usize = 10000;
const SCROLLBAR_THUMB_WIDTH: Pixels = px(8.);
const SCROLLBAR_THUMB_HEIGHT: Pixels = px(100.);

pub struct DataTable {
    state: Entity<State>,
    /// Use `Rc` to share the same quote data across multiple items, avoid cloning.
    // nodes: Vec<Rc<Node>>,
    visible_range: Range<usize>,
    scroll_handle: UniformListScrollHandle,
    /// The position in thumb bounds when dragging start mouse down.
    drag_position: Option<Point<Pixels>>,
}

impl DataTable {
    pub fn new(state: Entity<State>) -> Self {
        Self {
            state,
            // nodes: Vec::new(),
            visible_range: 0..0,
            scroll_handle: UniformListScrollHandle::new(),
            drag_position: None,
        }
    }

    fn table_bounds(&self) -> Bounds<Pixels> {
        self.scroll_handle.0.borrow().base_handle.bounds()
    }

    fn scroll_top(&self) -> Pixels {
        self.scroll_handle.0.borrow().base_handle.offset().y
    }

    fn scroll_height(&self) -> Pixels {
        self.scroll_handle
            .0
            .borrow()
            .last_item_size
            .unwrap_or_default()
            .contents
            .height
    }

    fn render_scrollbar(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scroll_height = self.scroll_height();
        let table_bounds = self.table_bounds();
        let table_height = table_bounds.size.height;
        if table_height == px(0.) {
            return div().id("scrollbar");
        }

        let percentage = -self.scroll_top() / scroll_height;
        let offset_top = (table_height * percentage).clamp(
            px(4.),
            (table_height - SCROLLBAR_THUMB_HEIGHT - px(4.)).max(px(4.)),
        );
        let entity = cx.entity();
        let scroll_handle = self.scroll_handle.0.borrow().base_handle.clone();

        div()
            .id("scrollbar")
            .absolute()
            .top(offset_top)
            .right_1()
            .h(SCROLLBAR_THUMB_HEIGHT)
            .w(SCROLLBAR_THUMB_WIDTH)
            .bg(rgb(0xC0C0C0))
            .hover(|this| this.bg(rgb(0xA0A0A0)))
            .rounded_lg()
            .child(
                canvas(
                    |_, _, _| (),
                    move |thumb_bounds, _, window, _| {
                        window.on_mouse_event({
                            let entity = entity.clone();
                            move |ev: &MouseDownEvent, _, _, cx| {
                                if !thumb_bounds.contains(&ev.position) {
                                    return;
                                }

                                entity.update(cx, |this, _| {
                                    this.drag_position = Some(
                                        ev.position - thumb_bounds.origin - table_bounds.origin,
                                    );
                                })
                            }
                        });
                        window.on_mouse_event({
                            let entity = entity.clone();
                            move |_: &MouseUpEvent, _, _, cx| {
                                entity.update(cx, |this, _| {
                                    this.drag_position = None;
                                })
                            }
                        });

                        window.on_mouse_event(move |ev: &MouseMoveEvent, _, _, cx| {
                            if !ev.dragging() {
                                return;
                            }

                            let Some(drag_pos) = entity.read(cx).drag_position else {
                                return;
                            };

                            let inside_offset = drag_pos.y;
                            let percentage = ((ev.position.y - table_bounds.origin.y
                                + inside_offset)
                                / (table_bounds.size.height))
                                .clamp(0., 1.);

                            let offset_y = ((scroll_height - table_bounds.size.height)
                                * percentage)
                                .clamp(px(0.), scroll_height - SCROLLBAR_THUMB_HEIGHT);
                            scroll_handle.set_offset(point(px(0.), -offset_y));
                            cx.notify(entity.entity_id());
                        })
                    },
                )
                .size_full(),
            )
    }
}

impl Render for DataTable {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        div()
            // .font_family(".SystemUIFont")
            // .bg(gpui::red())
            .text_sm()
            .size_full()
            // .p_4()
            // .gap_2()
            .flex()
            .flex_col()
            // // TODO
            // .child(format!(
            //     "Total {} items, visible range: {:?}",
            //     self.state.read(cx).nodes().len(),
            //     self.visible_range
            // ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_hidden()
                    .border_1()
                    .border_color(rgb(0xE0E0E0))
                    // .rounded_sm()
                    .child(render_titles())
                    .child(
                        div()
                            .relative()
                            .size_full()
                            .child(
                                // TODO: Is length reactive
                                uniform_list(entity, "items", self.state.read(cx).nodes().len(), {
                                    move |this, range, _, cx| {
                                        this.visible_range = range.clone();
                                        let mut items = Vec::with_capacity(range.end - range.start);
                                        let mut nodes = this.state.read(cx).nodes().iter();
                                        for i in range {
                                            if let Some(node) = nodes.next() {
                                                items.push(TableRow::new(i, this.state.clone()));
                                            }
                                        }

                                        items
                                    }
                                })
                                .size_full()
                                .track_scroll(self.scroll_handle.clone()),
                            )
                            .child(self.render_scrollbar(window, cx)),
                    ),
            )
    }
}
