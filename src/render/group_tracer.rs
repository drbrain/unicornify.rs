use crate::geometry::Vector;
use crate::render::Bounds;
use crate::render::FacetTracer;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;

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
        rendering_parameters: RenderingParameters,
        facet_tracer: &mut FacetTracer,
    ) {
        if !rendering_parameters.contains(self.bounds.clone()) {
            return;
        }

        for tracer in self.tracers.iter() {
            match tracer {
                Tracer::GroupT(t) => {
                    t.flatten_into_facets(rendering_parameters.clone(), facet_tracer);
                }
                _ => flatten_non_group_into_facets(
                    tracer.clone(),
                    rendering_parameters.clone(),
                    facet_tracer,
                ),
            }
        }
    }

    pub fn push(&mut self, tracer: Tracer) {
        self.tracers.push(tracer);

        self.bounds = self
            .tracers
            .iter()
            .map(|t| t.bounds())
            .fold(Bounds::empty(), |a, b| a.union(b));

        self.tracers.sort_by(|a, b| a.bounds().cmp(&b.bounds()));
    }

    pub fn prune(&self, rendering_parameters: RenderingParameters) -> Option<Tracer> {
        if rendering_parameters.contains(self.bounds.clone()) {
            return None;
        }

        let bounds = if rendering_parameters.x_min.is_infinite()
            || rendering_parameters.x_max.is_infinite()
            || rendering_parameters.y_min.is_infinite()
            || rendering_parameters.y_max.is_infinite()
        {
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

        let mut result = FacetTracer::new(bounds, 16);
        self.flatten_into_facets(rendering_parameters, &mut result);

        if result.is_empty() {
            return None;
        }

        Some(Tracer::FacetT(result))
    }

    pub fn trace(&self, x: f64, y: f64, ray: Vector) -> TraceResult {
        todo!("Implement BoneTracer.trace()");
    }
}

fn flatten_non_group_into_facets(
    tracer: Tracer,
    rendering_parameters: RenderingParameters,
    facet_tracer: &mut FacetTracer,
) {
    match tracer.prune(rendering_parameters.clone()) {
        None => {
            return;
        }
        Some(pruned) => match pruned {
            Tracer::GroupT(t) => t.flatten_into_facets(rendering_parameters, facet_tracer),
            Tracer::BoneT(_) => facet_tracer.add(pruned.clone()),
            Tracer::FacetT(_) => facet_tracer.add(pruned.clone()),
            Tracer::ScalingT(_) => facet_tracer.add(pruned.clone()),
            Tracer::TranslatingT(_) => facet_tracer.add(pruned.clone()),
        },
    }
}
