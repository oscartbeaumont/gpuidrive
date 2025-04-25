use gpui::*;

mod window;

actions!(example, [QuitApp]);

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        cx.bind_keys([KeyBinding::new("cmd-q", QuitApp, None)]);
        cx.on_action(|_: &QuitApp, cx| cx.quit());

        cx.bind_keys([KeyBinding::new("cmd-w", window::CloseWindow, None)]);
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

                    window::MainWindow { focus_handle }
                })
            },
        )
        .unwrap();
    });
}
