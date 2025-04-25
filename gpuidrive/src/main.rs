use gpui::*;

mod data_table;
mod window;

actions!(example, [QuitApp]);

fn main() {
    Application::new().run(|cx: &mut App| {
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
                focus: true,
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1280.0), px(1000.0)),
                    cx,
                ))),
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
