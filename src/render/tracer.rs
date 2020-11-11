use crate::render::BoneTracer;
use crate::render::GroupTracer;
use crate::render::ScalingTracer;
use crate::render::TranslatingTracer;
use crate::render::Bounds;

#[derive(Clone, Debug)]
pub enum Tracer {
    BoneT(BoneTracer),
    GroupT(GroupTracer),
    ScalingT(ScalingTracer),
    TranslatingT(TranslatingTracer),
}

impl Tracer {
    pub fn bounds(&self) -> Bounds {
        match self {
            Tracer::BoneT(t) => t.bounds.clone(),
            Tracer::GroupT(t) => t.bounds.clone(),
            Tracer::ScalingT(t) => t.bounds.clone(),
            Tracer::TranslatingT(t) => t.bounds.clone(),
        }
    }
}
