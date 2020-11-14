use crate::geometry::Vector;
use crate::render::prune_bounds;
use crate::render::Bounds;
use crate::render::GroupTracer;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct FacetTracer {
    root_count: usize,
    root_count_f: f64,
    facets: Vec<Option<Rc<RefCell<GroupTracer>>>>,
    pub bounds: Bounds,
    empty: bool,
}

impl FacetTracer {
    pub fn new(bounds: &Bounds, root_count: usize) -> Self {
        let root_count_f = root_count as f64;
        let facets = vec![None; root_count * root_count];
        let empty = true;
        let bounds = bounds.clone();

        FacetTracer {
            root_count,
            root_count_f,
            facets,
            bounds,
            empty,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn add(&mut self, tracer: Tracer) {
        self.empty = false;

        let bounds = tracer.bounds();

        let (min_x, min_y) = self.facet_coords(bounds.x_min, bounds.y_min);
        let (max_x, max_y) = self.facet_coords(bounds.x_max, bounds.y_max);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let n = y * self.root_count + x;

                let facet = match &self.facets[n] {
                    None => {
                        let t = Rc::new(RefCell::new(GroupTracer::new()));
                        self.facets[n] = Some(t.clone());
                        t
                    }
                    Some(t) => t.clone(),
                };

                facet.borrow_mut().push(tracer.clone());
            }
        }
    }

    fn facet_coords(&self, x: f64, y: f64) -> (usize, usize) {
        let b = self.bounds.clone();
        let root_count_f = self.root_count_f;

        let facet_x = (root_count_f - 1.0).min(0.0f64.max(root_count_f * (x - b.x_min) / b.dx()));
        let facet_y = (root_count_f - 1.0).min(0.0f64.max(root_count_f * (y - b.y_min) / b.dy()));

        (facet_x.trunc() as usize, facet_y.trunc() as usize)
    }

    fn facet_num(&self, x: f64, y: f64) -> usize {
        let (x, y) = self.facet_coords(x, y);

        y * self.root_count + x
    }

    pub fn prune(&self, rendering_parameters: RenderingParameters) -> Option<Tracer> {
        prune_bounds(Tracer::FacetT(self.clone()), rendering_parameters)
    }

    pub fn trace(&self, x: f64, y: f64, ray: Vector) -> TraceResult {
        match &self.facets[self.facet_num(x, y)] {
            None => None,
            Some(t) => t.borrow().trace(x, y, ray),
        }
    }
}
