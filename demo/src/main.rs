use std::{path::Path, sync::Arc, time::Duration};

use anyhow::anyhow;
use gpui::{
    Animation, AnimationExt, App, Application, Asset, AssetLogger, AssetSource, Bounds, Context,
    FocusHandle, Hsla, ImageAssetLoader, ImageCacheError, ImgResourceLoader, KeyBinding,
    LOADING_DELAY, Length, Pixels, RenderImage, Resource, SharedString, Window, WindowBounds,
    WindowOptions, actions, black, div, img, prelude::*, pulsating_between, px, red, rgb, size,
};

actions!(example, [CloseWindow]);

struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().to_string(),
                ))
            })
            .collect::<Vec<_>>())
    }
}

const IMAGE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/logo.png");

#[derive(Copy, Clone, Hash)]
struct LoadImageParameters {
    timeout: Duration,
    fail: bool,
}

struct LoadImageWithParameters {}

impl Asset for LoadImageWithParameters {
    type Source = LoadImageParameters;

    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        parameters: Self::Source,
        cx: &mut App,
    ) -> impl std::future::Future<Output = Self::Output> + Send + 'static {
        let timer = cx.background_executor().timer(parameters.timeout);
        let data = AssetLogger::<ImageAssetLoader>::load(
            Resource::Path(Path::new(IMAGE).to_path_buf().into()),
            cx,
        );
        async move {
            timer.await;
            if parameters.fail {
                println!("Intentionally failed to load image");
                Err(anyhow!("Failed to load image").into())
            } else {
                data.await
            }
        }
    }
}

struct ImageLoadingExample {}

impl ImageLoadingExample {
    fn loading_element() -> impl IntoElement {
        div().size_full().flex_none().p_0p5().rounded_xs().child(
            div().size_full().with_animation(
                "loading-bg",
                Animation::new(Duration::from_secs(3))
                    .repeat()
                    .with_easing(pulsating_between(0.04, 0.24)),
                move |this, delta| this.bg(black().opacity(delta)),
            ),
        )
    }

    fn fallback_element() -> impl IntoElement {
        let fallback_color: Hsla = black().opacity(0.5);

        div().size_full().flex_none().p_0p5().child(
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .rounded_xs()
                .text_sm()
                .text_color(fallback_color)
                .border_1()
                .border_color(fallback_color)
                .child("?"),
        )
    }
}

struct ExampleWindow {
    focus_handle: FocusHandle,
}

impl Render for ImageLoadingExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().flex_col().size_full().justify_around().child(
            div().flex().flex_row().w_full().justify_around().child(
                div()
                    .flex()
                    .bg(gpui::white())
                    .size(Length::Definite(Pixels(300.0).into()))
                    .justify_center()
                    .items_center()
                    .child({
                        let image_source = LoadImageParameters {
                            timeout: LOADING_DELAY.saturating_sub(Duration::from_millis(25)),
                            fail: false,
                        };

                        // Load within the 'loading delay', should not show loading fallback
                        img(move |window: &mut Window, cx: &mut App| {
                            window.use_asset::<LoadImageWithParameters>(&image_source, cx)
                        })
                        .id("image-1")
                        .border_1()
                        .size_12()
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .border_color(red())
                        .with_loading(|| Self::loading_element().into_any_element())
                        .on_click(move |_, _, cx| {
                            cx.remove_asset::<LoadImageWithParameters>(&image_source);
                        })
                    })
                    .child({
                        // Load after a long delay
                        let image_source = LoadImageParameters {
                            timeout: Duration::from_secs(5),
                            fail: false,
                        };

                        img(move |window: &mut Window, cx: &mut App| {
                            window.use_asset::<LoadImageWithParameters>(&image_source, cx)
                        })
                        .id("image-2")
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .with_loading(|| Self::loading_element().into_any_element())
                        .size_12()
                        .border_1()
                        .border_color(red())
                        .on_click(move |_, _, cx| {
                            cx.remove_asset::<LoadImageWithParameters>(&image_source);
                        })
                    })
                    .child({
                        // Fail to load image after a long delay
                        let image_source = LoadImageParameters {
                            timeout: Duration::from_secs(5),
                            fail: true,
                        };

                        // Fail to load after a long delay
                        img(move |window: &mut Window, cx: &mut App| {
                            window.use_asset::<LoadImageWithParameters>(&image_source, cx)
                        })
                        .id("image-3")
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .with_loading(|| Self::loading_element().into_any_element())
                        .size_12()
                        .border_1()
                        .border_color(red())
                        .on_click(move |_, _, cx| {
                            cx.remove_asset::<LoadImageWithParameters>(&image_source);
                        })
                    })
                    .child({
                        // Ensure that the normal image loader doesn't spam logs
                        let image_source = Path::new(
                            "this/file/really/shouldn't/exist/or/won't/be/an/image/I/hope",
                        )
                        .to_path_buf();
                        img(image_source.clone())
                            .id("image-1")
                            .border_1()
                            .size_12()
                            .with_fallback(|| Self::fallback_element().into_any_element())
                            .border_color(red())
                            .with_loading(|| Self::loading_element().into_any_element())
                            .on_click(move |_, _, cx| {
                                cx.remove_asset::<ImgResourceLoader>(&image_source.clone().into());
                            })
                    }),
            ),
        )
    }
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
            .child(button("Resize", |window, _| {
                // let content_size = window.bounds().size;
                // window.resize(size(content_size.height, content_size.width));
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
                    // cx.observe_window_bounds(window, move |_, window, _| {
                    //     println!("Window bounds changed: {:?}", window.bounds());
                    //     let content_size = window.bounds().size;
                    //     window.resize(size(content_size.height, content_size.width))
                    // })
                    // .detach();

                    let focus_handle = cx.focus_handle();
                    focus_handle.focus(window);

                    // ExampleWindow { focus_handle }
                    ImageLoadingExample {}
                })
            },
        )
        .unwrap();
    });
}
