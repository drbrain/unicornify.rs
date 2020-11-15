use crate::geometry::Vector;
use crate::render::BoneTracer;
use crate::render::Bounds;
use crate::render::FacetTracer;
use crate::render::GroupTracer;
use crate::render::RenderingParameters;
use crate::render::ScalingTracer;
use crate::render::TraceResult;
use crate::render::TranslatingTracer;
use crate::render::WorldView;

use image::RgbaImage;

#[derive(Clone, Debug, PartialEq)]
pub enum Tracer {
    BoneT(BoneTracer),
    FacetT(FacetTracer),
    GroupT(GroupTracer),
    ScalingT(ScalingTracer),
    TranslatingT(TranslatingTracer),
}

impl Tracer {
    pub fn bounds(&self) -> Bounds {
        match self {
            Tracer::BoneT(t) => t.bounds.clone(),
            Tracer::FacetT(t) => t.bounds.clone(),
            Tracer::GroupT(t) => t.bounds.clone(),
            Tracer::ScalingT(t) => t.bounds.clone(),
            Tracer::TranslatingT(t) => t.bounds.clone(),
        }
    }

    pub fn draw(&mut self, world_view: WorldView, image_buffer: &mut RgbaImage) {
        let bounds = image_buffer.into();

        self.draw_partial(world_view, image_buffer, &bounds);
    }

    pub fn draw_partial(
        &mut self,
        world_view: WorldView,
        image_buffer: &mut RgbaImage,
        bounds: &Bounds,
    ) {
        let bounds = self.bounds();
        eprintln!("partial_{:?}", self.bounds());
        let rect = bounds.intersection(&self.bounds());

        let rendering_parameters = RenderingParameters::new(1.0, rect.clone());

        match self.prune(&rendering_parameters) {
            Some(pruned) => {
                let x_min = rect.clone().x_min as u32;
                let x_max = rect.clone().x_max as u32;
                let y_min = rect.clone().y_min as u32;
                let y_max = rect.clone().y_max as u32;

                for y in x_min..x_max {
                    for x in y_min..y_max {
                        let fx = x as f64;
                        let fy = y as f64;
                        let ray = world_view.ray(fx, fy);

                        match pruned.trace(fx, fy, ray) {
                            Some((_, _, color)) => {
                                image_buffer.put_pixel(x, y, color.into());
                            }
                            None => (),
                        }
                    }
                }
            }
            None => (),
        }
    }

    pub fn prune(&self, rendering_parameters: &RenderingParameters) -> Option<Tracer> {
        match self {
            Tracer::BoneT(t) => t.prune(rendering_parameters),
            Tracer::FacetT(t) => t.prune(rendering_parameters),
            Tracer::GroupT(t) => t.prune(rendering_parameters),
            Tracer::ScalingT(t) => t.prune(rendering_parameters),
            Tracer::TranslatingT(t) => t.prune(rendering_parameters),
        }
    }

    pub fn trace(&self, x: f64, y: f64, ray: Vector) -> TraceResult {
        match self {
            Tracer::BoneT(t) => t.trace(x, y, ray),
            Tracer::FacetT(t) => t.trace(x, y, ray),
            Tracer::GroupT(t) => t.trace(x, y, ray),
            Tracer::ScalingT(t) => t.trace(x, y, ray),
            Tracer::TranslatingT(t) => t.trace(x, y, ray),
        }
    }
}
