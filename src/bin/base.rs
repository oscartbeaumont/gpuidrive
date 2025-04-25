use std::{path::Path, sync::Arc, time::Duration};

use anyhow::anyhow;
use gpui::{
    Animation, AnimationExt, App, Application, Asset, AssetLogger, AssetSource, Bounds, Context,
    FocusHandle, Hsla, ImageAssetLoader, ImageCacheError, ImgResourceLoader, KeyBinding,
    LOADING_DELAY, Length, Pixels, RenderImage, Resource, SharedString, Window, WindowBounds,
    WindowOptions, actions, black, div, img, prelude::*, pulsating_between, px, red, rgb, size,
};

actions!(example, [CloseWindow]);

struct ExampleWindow {
    focus_handle: FocusHandle,
}

impl Render for ExampleWindow {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(|_: &CloseWindow, window, _| {
                window.remove_window();
            })
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(
                "Closing this window with cmd-w or the traffic lights should quit the application!",
            )
            .child(button("Testing", |window, _| {
                println!("Hello World!");
            }))
    }
}

fn button(text: &str, on_click: impl Fn(&mut Window, &mut App) + 'static) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_2()
        .bg(rgb(0xf7f7f7))
        .active(|this| this.opacity(0.85))
        .border_1()
        .border_color(rgb(0xe0e0e0))
        .rounded_sm()
        .cursor_pointer()
        .child(text.to_string())
        .on_click(move |_, window, cx| on_click(window, cx))
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        cx.bind_keys([KeyBinding::new("cmd-w", CloseWindow, None)]);
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.activate(false);
                cx.new(|cx| {
                    let focus_handle = cx.focus_handle();
                    focus_handle.focus(window);

                    ExampleWindow { focus_handle }
                })
            },
        )
        .unwrap();
    });
}
