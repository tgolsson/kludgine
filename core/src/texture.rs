use std::{
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use image::{DynamicImage, RgbaImage};
use lazy_static::lazy_static;
use winit::window::Icon;

use crate::math::Size;

lazy_static! {
    static ref GLOBAL_ID_CELL: AtomicU64 = AtomicU64::new(0);
}

#[macro_export]
macro_rules! include_texture {
    ($image_path:expr) => {{
        let image_bytes = std::include_bytes!($image_path);
        Texture::from_bytes(image_bytes)
    }};
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u64,
    pub image: Arc<RgbaImage>,
}

impl Texture {
    #[must_use]
    pub fn new(image: &DynamicImage) -> Self {
        let image = image.to_rgba8();
        let id = GLOBAL_ID_CELL.fetch_add(1, Ordering::SeqCst);
        Self {
            id,
            image: Arc::new(image),
        }
    }

    pub fn load<P: AsRef<Path>>(from_path: P) -> crate::Result<Self> {
        let img = image::open(from_path)?;

        Ok(Self::new(&img))
    }

    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        let img = image::load_from_memory(bytes)?;

        Ok(Self::new(&img))
    }

    #[must_use]
    pub fn size(&self) -> Size<u32> {
        let (w, h) = self.image.dimensions();
        Size::new(w as u32, h as u32)
    }

    #[must_use]
    pub fn rgba_pixels(&self) -> Vec<u8> {
        (*self.image).clone().into_vec()
    }

    pub fn window_icon(&self) -> Result<Icon, winit::window::BadIcon> {
        Icon::from_rgba(self.rgba_pixels(), self.image.width(), self.image.height())
    }
}
