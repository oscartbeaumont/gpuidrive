use gpui::*;
use input::TextInput;

mod data_table;
mod input;
mod state;
mod window;

actions!(example, [QuitApp]);

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            // Window actions
            KeyBinding::new("cmd-q", QuitApp, None),
            KeyBinding::new("cmd-w", window::CloseWindow, None),
            // Input
            KeyBinding::new("backspace", input::Backspace, None),
            KeyBinding::new("delete", input::Delete, None),
            KeyBinding::new("left", input::Left, None),
            KeyBinding::new("right", input::Right, None),
            KeyBinding::new("shift-left", input::SelectLeft, None),
            KeyBinding::new("shift-right", input::SelectRight, None),
            KeyBinding::new("cmd-a", input::SelectAll, None),
            KeyBinding::new("cmd-v", input::Paste, None),
            KeyBinding::new("cmd-c", input::Copy, None),
            KeyBinding::new("cmd-x", input::Cut, None),
            KeyBinding::new("home", input::Home, None),
            KeyBinding::new("end", input::End, None),
            KeyBinding::new("ctrl-cmd-space", input::ShowCharacterPalette, None),
        ]);

        cx.on_action(|_: &QuitApp, cx| cx.quit());
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let window = cx
            .open_window(
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
                    cx.new(|cx| window::MainWindow::init(cx, window))
                },
            )
            .unwrap();

        let view = window.update(cx, |_, _, cx| cx.entity()).unwrap();
        cx.observe_keystrokes(move |ev, _, cx| {
            view.update(cx, |view, cx| {
                // view.recent_keystrokes.push(ev.keystroke.clone());
                cx.notify();
            })
        })
        .detach();
        cx.on_keyboard_layout_change({
            move |cx| {
                window.update(cx, |_, _, cx| cx.notify()).ok();
            }
        })
        .detach();

        // TODO
        // window
        //     .update(cx, |view, window, cx| {
        //         window.focus(&view.text_input.focus_handle(cx));
        //         cx.activate(true);
        //     })
        //     .unwrap();
    });
}
