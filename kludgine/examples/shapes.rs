use kludgine::prelude::*;

fn main() {
    SingleWindowApplication::run(Shapes::default());
}

#[derive(Default)]
struct Shapes;

impl WindowCreator for Shapes {
    fn window_title(&self) -> String {
        "Shapes - Kludgine".to_owned()
    }
}

impl Window for Shapes {
    fn render(
        &mut self,
        scene: &Target,
        _status: &mut RedrawStatus,
        _window: WindowHandle,
    ) -> kludgine::Result<()> {
        let center = Rect::new(Point::default(), scene.size()).center();

        Shape::<Scaled>::polygon(vec![
            Point::new(-100., -100.),
            Point::new(0., 100.),
            Point::new(100., -100.),
        ])
        .fill(Fill::new(Color::GREEN))
        .render_at(&center, scene);

        Shape::<Scaled>::circle(Point::new(0., 0.), Figure::new(25.))
            .fill(Fill::new(Color::RED))
            .render_at(&center, scene);

        Ok(())
    }
}
