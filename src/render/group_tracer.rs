use crate::geometry::Vector;
use crate::render::Bounds;
use crate::render::FacetTracer;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;
use crate::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct GroupTracer {
    tracers: Vec<Tracer>,
    pub bounds: Bounds,
}

impl GroupTracer {
    pub fn new() -> Self {
        let tracers: Vec<Tracer> = Vec::new();
        let bounds = Bounds::empty();

        GroupTracer { tracers, bounds }
    }

    pub fn flatten_into_facets(
        &self,
        rendering_parameters: &RenderingParameters,
        facet_tracer: &mut FacetTracer,
    ) {
        if !rendering_parameters.contains(&self.bounds) {
            return;
        }

        for tracer in self.tracers.iter() {
            match tracer {
                Tracer::GroupT(t) => {
                    t.flatten_into_facets(rendering_parameters, facet_tracer);
                }
                _ => flatten_non_group_into_facets(
                    tracer.clone(),
                    rendering_parameters,
                    facet_tracer,
                ),
            }
        }
    }

    pub fn add(&mut self, tracer: Tracer) {
        let bounds = tracer.bounds();

        self.bounds = self.bounds.union(&bounds);

        let index = match self.tracers.binary_search_by(|b| b.bounds().cmp(&bounds)) {
            Ok(i) => i,
            Err(i) => i,
        };

        self.tracers.insert(index, tracer);
    }

    pub fn prune(&self, rendering_parameters: &RenderingParameters) -> Option<Tracer> {
        if !rendering_parameters.contains(&self.bounds) {
            eprintln!("Group out of bounds");
            return None;
        }

        let bounds = if rendering_parameters.any_infinite() {
            self.bounds.clone()
        } else {
            Bounds {
                x_min: rendering_parameters.x_min,
                x_max: rendering_parameters.x_max,
                y_min: rendering_parameters.y_min,
                y_max: rendering_parameters.y_max,
                z_min: self.bounds.z_min,
                z_max: self.bounds.z_max,
                empty: false,
            }
        };

        let mut result = FacetTracer::new(&bounds, 16);
        self.flatten_into_facets(rendering_parameters, &mut result);

        if result.is_empty() {
            return None;
        }

        Some(Tracer::FacetT(result))
    }

    pub fn trace(&self, x: f64, y: f64, ray: Vector) -> TraceResult {
        let mut any = false;
        let mut min_z: f64 = 0.0;
        let mut color = Color::black();
        let mut dir = Vector::zero();

        for tracer in self.tracers.iter() {
            let bounds = tracer.bounds();

            if bounds.z_max <= 0.0 {
                continue;
            }

            if !bounds.contains_xy(x, y) {
                continue;
            }

            if any && !bounds.contains_points_in_front_of_z(min_z) {
                break;
            }

            match tracer.trace(x, y, ray) {
                Some((z, t_dir, t_color)) => {
                    if z > 0.0 {
                        if !any || z < min_z {
                            color = t_color;
                            min_z = z;
                            dir = t_dir;
                            any = true;
                        }
                    }
                }
                None => (),
            };
        }

        if any {
            Some((min_z, dir, color))
        } else {
            None
        }
    }
}

fn flatten_non_group_into_facets(
    tracer: Tracer,
    rendering_parameters: &RenderingParameters,
    facet_tracer: &mut FacetTracer,
) {
    match tracer.prune(rendering_parameters) {
        None => {
            return;
        }
        Some(pruned) => match pruned {
            Tracer::GroupT(t) => t.flatten_into_facets(rendering_parameters, facet_tracer),
            Tracer::BoneT(_) => facet_tracer.add(pruned.clone()),
            Tracer::FacetT(_) => facet_tracer.add(pruned.clone()),
            Tracer::QuadrantT(_) => facet_tracer.add(pruned.clone()),
            Tracer::ScalingT(_) => facet_tracer.add(pruned.clone()),
            Tracer::TranslatingT(_) => facet_tracer.add(pruned.clone()),
        },
    }
}
