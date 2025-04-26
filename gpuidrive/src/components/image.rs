// use std::{path::PathBuf, time::Duration};

// use gpui::*;

// use crate::assets::{LoadImageParameters, LoadImageWithParameters};

// pub struct ImageLoadingExample {}

// impl ImageLoadingExample {
//     fn loading_element() -> impl IntoElement {
//         div().size_full().flex_none().p_0p5().rounded_xs().child(
//             div().size_full().with_animation(
//                 "loading-bg",
//                 Animation::new(Duration::from_secs(3))
//                     .repeat()
//                     .with_easing(pulsating_between(0.04, 0.24)),
//                 move |this, delta| this.bg(black().opacity(delta)),
//             ),
//         )
//     }

//     fn fallback_element() -> impl IntoElement {
//         let fallback_color: Hsla = black().opacity(0.5);

//         div().size_full().flex_none().p_0p5().child(
//             div()
//                 .size_full()
//                 .flex()
//                 .items_center()
//                 .justify_center()
//                 .rounded_xs()
//                 .text_sm()
//                 .text_color(fallback_color)
//                 .border_1()
//                 .border_color(fallback_color)
//                 .child("?"),
//         )
//     }
// }

// impl Render for ImageLoadingExample {
//     fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
//         div().flex().flex_col().size_full().justify_around().child(
//             div().flex().flex_row().w_full().justify_around().child(
//                 div()
//                     .flex()
//                     .bg(gpui::white())
//                     .size(Length::Definite(Pixels(300.0).into()))
//                     .justify_center()
//                     .items_center()
//                     .child({
//                         let image_source = LoadImageParameters {
//                             path: PathBuf::from("/Users/oscar/Desktop/sdtest/logo.png"),
//                             timeout: LOADING_DELAY.saturating_sub(Duration::from_millis(25)),
//                             fail: false,
//                         };

//                         // Load within the 'loading delay', should not show loading fallback
//                         img(move |window: &mut Window, cx: &mut App| {
//                             window.use_asset::<LoadImageWithParameters>(&image_source, cx)
//                         })
//                         .id("image-1")
//                         .border_1()
//                         .size_12()
//                         .with_fallback(|| Self::fallback_element().into_any_element())
//                         .border_color(red())
//                         .with_loading(|| Self::loading_element().into_any_element())
//                         .on_click(move |_, _, cx| {
//                             cx.remove_asset::<LoadImageWithParameters>(&image_source);
//                         })
//                     })
//                     .child({
//                         // Load after a long delay
//                         let image_source = LoadImageParameters {
//                             timeout: Duration::from_secs(5),
//                             fail: false,
//                         };

//                         img(move |window: &mut Window, cx: &mut App| {
//                             window.use_asset::<LoadImageWithParameters>(&image_source, cx)
//                         })
//                         .id("image-2")
//                         .with_fallback(|| Self::fallback_element().into_any_element())
//                         .with_loading(|| Self::loading_element().into_any_element())
//                         .size_12()
//                         .border_1()
//                         .border_color(red())
//                         .on_click(move |_, _, cx| {
//                             cx.remove_asset::<LoadImageWithParameters>(&image_source);
//                         })
//                     })
//                     .child({
//                         // Fail to load image after a long delay
//                         let image_source = LoadImageParameters {
//                             timeout: Duration::from_secs(5),
//                             fail: true,
//                         };

//                         // Fail to load after a long delay
//                         img(move |window: &mut Window, cx: &mut App| {
//                             window.use_asset::<LoadImageWithParameters>(&image_source, cx)
//                         })
//                         .id("image-3")
//                         .with_fallback(|| Self::fallback_element().into_any_element())
//                         .with_loading(|| Self::loading_element().into_any_element())
//                         .size_12()
//                         .border_1()
//                         .border_color(red())
//                         .on_click(move |_, _, cx| {
//                             cx.remove_asset::<LoadImageWithParameters>(&image_source);
//                         })
//                     })
//                     .child({
//                         // Ensure that the normal image loader doesn't spam logs
//                         let image_source = Path::new(
//                             "this/file/really/shouldn't/exist/or/won't/be/an/image/I/hope",
//                         )
//                         .to_path_buf();
//                         img(image_source.clone())
//                             .id("image-1")
//                             .border_1()
//                             .size_12()
//                             .with_fallback(|| Self::fallback_element().into_any_element())
//                             .border_color(red())
//                             .with_loading(|| Self::loading_element().into_any_element())
//                             .on_click(move |_, _, cx| {
//                                 cx.remove_asset::<ImgResourceLoader>(&image_source.clone().into());
//                             })
//                     }),
//             ),
//         )
//     }
// }
