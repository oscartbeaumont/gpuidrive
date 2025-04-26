use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::anyhow;
use gpui::{
    App, Asset, AssetLogger, AssetSource, ImageAssetLoader, ImageCacheError, RenderImage, Resource,
    SharedString,
};

pub struct Assets;

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

#[derive(Clone, Hash)]
pub struct LoadImageParameters {
    pub path: PathBuf,
    // TODO: Can probs remove these
    pub timeout: Duration,
    pub fail: bool,
}

pub struct LoadImageWithParameters {}

impl Asset for LoadImageWithParameters {
    type Source = LoadImageParameters;

    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        parameters: Self::Source,
        cx: &mut App,
    ) -> impl std::future::Future<Output = Self::Output> + Send + 'static {
        let timer = cx.background_executor().timer(parameters.timeout);
        let data =
            AssetLogger::<ImageAssetLoader>::load(Resource::Path(parameters.path.into()), cx);
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
