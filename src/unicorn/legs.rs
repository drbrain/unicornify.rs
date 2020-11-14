use crate::geometry::Axis;
use crate::geometry::Ball;
use crate::geometry::Vector;
use crate::render::GroupTracer;
use crate::render::WorldView;
use crate::unicorn::Leg;
use crate::Color;
use crate::Data;

#[derive(Clone, Debug)]
pub struct Legs {
    pub fr: Leg,
    pub fl: Leg,
    pub br: Leg,
    pub bl: Leg,
}

impl Legs {
    pub fn new(data: &Data, butt: &Ball, shoulder: &Ball) -> Self {
        let hip_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let knee_color = Color::hsl(data.body_hue, data.body_sat, 70);
        let hoof_color = Color::hsl(data.body_hue, data.body_sat, 45);

        let hip = Ball::new(
            "left front hip".into(),
            55.0,
            160.0,
            -25.0,
            25.0,
            hip_color.clone(),
        );
        let knee = Ball::new(
            "left front knee".into(),
            35.0,
            254.0,
            -25.0,
            9.0,
            knee_color.clone(),
        );
        let hoof = Ball::new(
            "left front hoof".into(),
            55.0,
            310.0,
            -25.0,
            11.0,
            hoof_color.clone(),
        );
        hip.move_to_sphere(shoulder);
        let fr = Leg::new(hip, knee, hoof);

        let hip = Ball::new(
            "right front hip".into(),
            55.0,
            160.0,
            25.0,
            25.0,
            hip_color.clone(),
        );
        let knee = Ball::new(
            "right front knee".into(),
            35.0,
            254.0,
            25.0,
            9.0,
            knee_color.clone(),
        );
        let hoof = Ball::new(
            "right front hoof".into(),
            55.0,
            310.0,
            25.0,
            11.0,
            hoof_color.clone(),
        );
        hip.move_to_sphere(shoulder);
        let fl = Leg::new(hip, knee, hoof);

        let hip = Ball::new(
            "left rear hip".into(),
            225.0,
            190.0,
            -25.0,
            25.0,
            hip_color.clone(),
        );
        let knee = Ball::new(
            "left right knee".into(),
            230.0,
            265.0,
            -25.0,
            9.0,
            knee_color.clone(),
        );
        let hoof = Ball::new(
            "left rear hoof".into(),
            220.0,
            310.0,
            -25.0,
            11.0,
            hoof_color.clone(),
        );
        hip.move_to_sphere(butt);
        let br = Leg::new(hip, knee, hoof);

        let hip = Ball::new(
            "right rear hip".into(),
            225.0,
            190.0,
            25.0,
            25.0,
            hip_color.clone(),
        );
        let knee = Ball::new(
            "right rear knee".into(),
            230.0,
            265.0,
            25.0,
            9.0,
            knee_color.clone(),
        );
        let hoof = Ball::new(
            "right rear hoof".into(),
            220.0,
            310.0,
            25.0,
            11.0,
            hoof_color.clone(),
        );
        hip.move_to_sphere(butt);
        let bl = Leg::new(hip, knee, hoof);

        let mut legs = Legs { fr, fl, br, bl };
        data.pose.pose(&mut legs);

        legs
    }

    pub fn add_traceable(&self, mut tracer: &mut GroupTracer, world_view: WorldView) {
        self.fr.add_traceable(&mut tracer, world_view.clone());
        self.fl.add_traceable(&mut tracer, world_view.clone());
        self.br.add_traceable(&mut tracer, world_view.clone());
        self.bl.add_traceable(&mut tracer, world_view);
    }

    pub fn rotate_around(&self, other: &Vector, angle: f64, axis: Axis) {
        self.fr.rotate_around(other, angle, axis);
        self.fl.rotate_around(other, angle, axis);
        self.br.rotate_around(other, angle, axis);
        self.bl.rotate_around(other, angle, axis);
    }
}
