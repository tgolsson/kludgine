use crate::{
    math::{Box2D, Point, Raw, Rect, Scaled, Size},
    scene::{Element, Target},
    sprite::{RenderedSprite, SpriteRotation},
    texture::Texture,
};
#[derive(Debug, Clone)]
pub struct SpriteSource {
    pub location: SpriteSourceLocation,
    pub texture: Texture,
}

#[derive(Debug, Clone)]
pub enum SpriteSourceLocation {
    Rect(Rect<u32>),
    Joined(Vec<SpriteSourceSublocation>),
}

impl SpriteSourceLocation {
    #[must_use]
    pub fn bounds(&self) -> Rect<u32> {
        match self {
            Self::Rect(rect) => *rect,
            Self::Joined(locations) => locations
                .iter()
                .fold(Option::<Rect<u32>>::None, |union, location| {
                    Some(union.map_or_else(
                        || location.destination_rect(),
                        |total| total.union(&location.destination_rect()),
                    ))
                })
                .unwrap_or_default(),
        }
    }

    #[must_use]
    pub fn size(&self) -> Size<u32> {
        self.bounds().size
    }
}

#[derive(Debug, Clone)]
pub struct SpriteSourceSublocation {
    pub source: Rect<u32>,
    pub destination: Point<u32>,
}

impl SpriteSourceSublocation {
    #[must_use]
    pub const fn destination_rect(&self) -> Rect<u32> {
        Rect::new(self.destination, self.source.size)
    }
}

impl SpriteSource {
    #[must_use]
    pub const fn new(location: Rect<u32>, texture: Texture) -> Self {
        Self {
            location: SpriteSourceLocation::Rect(location),
            texture,
        }
    }

    #[must_use]
    pub fn joined<I: IntoIterator<Item = SpriteSourceSublocation>>(
        locations: I,
        texture: Texture,
    ) -> Self {
        Self {
            location: SpriteSourceLocation::Joined(locations.into_iter().collect()),
            texture,
        }
    }

    /// All `SpriteSources` must be from the same texture, and must have a
    /// square number of sprites
    #[must_use]
    pub fn joined_square<I: IntoIterator<Item = Self>>(sources: I) -> Self {
        let sources: Vec<_> = sources.into_iter().collect();
        #[allow(clippy::cast_sign_loss)] // sqrt of a positive number is always positive
        let sprites_wide = (sources.len() as f32).sqrt() as usize;
        assert!(sprites_wide * sprites_wide == sources.len()); // check for square
        let texture = sources[0].texture.clone();

        let sprite_size = sources[0].location.bounds().size;
        let mut sources = sources.into_iter();
        let mut locations = Vec::new();
        for y in 0..sprites_wide {
            for x in 0..sprites_wide {
                let source = sources.next().unwrap();
                debug_assert!(texture.id == source.texture.id);
                locations.push(SpriteSourceSublocation {
                    source: source.location.bounds(),
                    destination: Point::new(
                        x as u32 * sprite_size.width,
                        y as u32 * sprite_size.height,
                    ),
                });
            }
        }

        Self::joined(locations, texture)
    }

    #[must_use]
    pub fn entire_texture(texture: Texture) -> Self {
        Self::new(Rect::new(Point::default(), texture.size()), texture)
    }

    pub fn render_at(
        &self,
        scene: &Target,
        location: Point<f32, Scaled>,
        rotation: SpriteRotation<Scaled>,
    ) {
        self.render_at_with_alpha(scene, location, rotation, 1.)
    }

    pub fn render_within(
        &self,
        scene: &Target,
        bounds: Rect<f32, Scaled>,
        rotation: SpriteRotation<Scaled>,
    ) {
        self.render_with_alpha(scene, bounds, rotation, 1.)
    }

    pub fn render_within_box(
        &self,
        scene: &Target,
        bounds: Box2D<f32, Scaled>,
        rotation: SpriteRotation<Scaled>,
    ) {
        self.render_with_alpha_in_box(scene, bounds, rotation, 1.)
    }

    pub fn render_at_with_alpha(
        &self,
        scene: &Target,
        location: Point<f32, Scaled>,
        rotation: SpriteRotation<Scaled>,
        alpha: f32,
    ) {
        self.render_with_alpha(
            scene,
            Rect::new(location, self.location.size().to_f32().cast_unit()),
            rotation,
            alpha,
        )
    }

    pub fn render_with_alpha(
        &self,
        scene: &Target,
        bounds: Rect<f32, Scaled>,
        rotation: SpriteRotation<Scaled>,
        alpha: f32,
    ) {
        self.render_with_alpha_in_box(scene, bounds.to_box2d(), rotation, alpha)
    }

    pub fn render_with_alpha_in_box(
        &self,
        scene: &Target,
        bounds: Box2D<f32, Scaled>,
        rotation: SpriteRotation<Scaled>,
        alpha: f32,
    ) {
        let effective_scale = scene.scale_factor();
        self.render_raw_with_alpha_in_box(
            scene,
            bounds * effective_scale,
            rotation * effective_scale,
            alpha,
        )
    }

    pub fn render_raw_with_alpha_in_box(
        &self,
        scene: &Target,
        bounds: Box2D<f32, Raw>,
        rotation: SpriteRotation<Raw>,
        alpha: f32,
    ) {
        let bounds = Box2D::new(
            scene.offset_point_raw(bounds.min),
            scene.offset_point_raw(bounds.max),
        );
        scene.push_element(Element::Sprite {
            sprite: RenderedSprite::new(bounds, rotation, alpha, self.clone()),
            clip: scene.clip,
        });
    }
}
