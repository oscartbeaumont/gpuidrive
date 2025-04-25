use gpui::*;

use crate::state::State;

pub struct Preview {
    visible: bool,
}

impl Preview {
    pub fn init(state: Entity<State>) -> Self {
        Self { visible: false }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }
}

impl Render for Preview {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        if !self.visible {
            return div();
        }

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
                    .child(div().w(relative(0.5)).h(relative(0.5)).bg(gpui::red())),
            )
    }
}
