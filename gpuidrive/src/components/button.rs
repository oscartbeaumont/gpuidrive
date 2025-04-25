use gpui::*;

pub fn button(text: &str, on_click: impl Fn(&mut Window, &mut App) + 'static) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        // .flex_none()
        // .px_2()
        // .bg(rgb(0xf7f7f7))
        // .active(|this| this.opacity(0.85))
        // .border_1()
        // .border_color(rgb(0xe0e0e0))
        // .rounded_sm()
        // .cursor_pointer()
        // .child(text.to_string())
        .on_click(move |_, window, cx| on_click(window, cx))
}
