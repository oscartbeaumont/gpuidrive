use std::path::PathBuf;

use gpui::{
    App, ImageSource, IntoElement, ParentElement, RenderOnce, Resource, SharedString, Styled,
    Window, black, div, img,
};

#[derive(IntoElement)]
pub enum Icon {
    PhFile,
    PhFolder,
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _: &mut App) -> impl IntoElement {
        // let todo = include_str!("../../icons/PhFile.svg");
        // println!("{:?}", todo);

        // let todo = ImageSource::Resource(Resource::Embedded(SharedString::new(todo)));
        //
        // let todo = ImageSource::Resource(Resource::Path(
        //     PathBuf::from("./gpuidrive/icon/PhFile.svg").into(),
        // ));

        // ./gpuidrive/icons/PhFile.svg"
        img("./gpuidrive/icons/PhFile.svg")
            .text_color(black())
            .size_8()
    }
}
