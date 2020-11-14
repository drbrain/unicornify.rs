use crate::geometry::Axis;
use crate::geometry::Ball;
use crate::geometry::Bone;
use crate::geometry::Gamma;
use crate::geometry::Vector;
use crate::render::GroupTracer;
use crate::render::WorldView;
use crate::unicorn::Head;
use crate::unicorn::Legs;
use crate::unicorn::Mane;
use crate::unicorn::Neck;
use crate::unicorn::Pose;
use crate::unicorn::Torso;
use crate::Color;
use crate::Data;

#[derive(Clone, Debug)]
pub struct Unicorn {
    torso: Torso,
}

impl Unicorn {
    pub fn new(data: &Data) -> Self {
        let head_color = Color::hsl(data.body_hue, data.body_sat, 60);
        let head = Ball::new("head".into(), 0.0, 0.0, 0.0, data.head_size, head_color);

        let snout_color = Color::hsl(data.body_hue, data.body_sat, 80);
        let snout = Ball::new(
            "snout".into(),
            -25.0,
            60.0,
            0.0,
            data.snout_size,
            snout_color,
        );
        snout.set_distance(data.snout_length, &head);

        let shoulder_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let shoulder = Ball::new(
            "shoulder".into(),
            80.0,
            120.0,
            0.0,
            data.shoulder_size,
            shoulder_color,
        );

        let butt_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let butt = Ball::new("butt".into(), 235.0, 155.0, 0.0, data.butt_size, butt_color);

        let horn_root_color = Color::hsl(data.horn_hue, data.horn_sat, 70);
        let horn_onset = Ball::new(
            "horn onset".into(),
            -22.0,
            -10.0,
            0.0,
            data.horn_onset_size,
            horn_root_color,
        );
        horn_onset.move_to_sphere(&head);

        let horn_tip_color = Color::hsl(data.horn_hue, data.horn_sat, 90);
        let tip_pos = horn_onset.clone() + Vector::new(-10.0, 0.0, 0.0);
        let horn_tip = Ball::new(
            "horn tip".into(),
            tip_pos.x,
            tip_pos.y,
            tip_pos.z,
            data.horn_tip_size,
            horn_tip_color,
        );
        horn_tip.set_distance(data.horn_length, &horn_onset);
        horn_tip.rotate_around(*horn_onset.center.borrow(), data.horn_angle, Axis::Z);

        let eye_left = Ball::new(
            "left eye".into(),
            -10.0,
            3.0,
            -5.0,
            data.eye_size,
            Color::white(),
        );
        eye_left.set_gap(5.0, &head);

        let eye_right = Ball::new(
            "right eye".into(),
            -10.0,
            3.0,
            5.0,
            data.eye_size,
            Color::white(),
        );
        eye_right.set_gap(5.0, &head);

        let pupil_left = Ball::new_v(
            "left pupil".into(),
            eye_left.clone() + Vector::new(-1.0, 0.0, 0.0),
            data.pupil_size,
            Color::black(),
        );
        pupil_left.move_to_sphere(&eye_left);

        let pupil_right = Ball::new_v(
            "right pupil".into(),
            eye_right.clone() + Vector::new(-1.0, 0.0, 0.0),
            data.pupil_size,
            Color::black(),
        );
        pupil_right.move_to_sphere(&eye_right);

        let mood_delta = data.brow_mood * 3.0;

        let brow_inner_color = Color::hsl(data.hair_hue, data.hair_sat, 50);
        let brow_middle_color = Color::hsl(data.hair_hue, data.hair_sat, 70);
        let brow_outer_color = Color::hsl(data.hair_hue, data.hair_sat, 60);

        let brow_left_inner = Ball::new_v(
            "left inner brow".into(),
            eye_left.clone() + Vector::new(0.0, -10.0, data.brow_length),
            data.brow_size,
            brow_inner_color.clone(),
        );
        brow_left_inner.set_gap(5.0 + mood_delta, &eye_left);

        let brow_left_middle = Ball::new_v(
            "left middle brow".into(),
            eye_left.clone() + Vector::new(0.0, -10.0, 0.0),
            data.brow_size,
            brow_middle_color.clone(),
        );
        brow_left_middle.set_gap(5.0 + data.brow_length, &eye_left);

        let brow_left_outer = Ball::new_v(
            "left outer brow".into(),
            eye_left.clone() + Vector::new(0.0, -10.0, -data.brow_length),
            data.brow_size,
            brow_outer_color.clone(),
        );
        brow_left_outer.set_gap(5.0 - mood_delta, &eye_left);

        let brow_right_inner = Ball::new_v(
            "right inner brow".into(),
            eye_right.clone() + Vector::new(0.0, -10.0, -data.brow_length),
            data.brow_size,
            brow_inner_color.clone(),
        );
        brow_right_inner.set_gap(5.0 + mood_delta, &eye_right);

        let brow_right_middle = Ball::new_v(
            "right middle brow".into(),
            eye_right.clone() + Vector::new(0.0, -10.0, 0.0),
            data.brow_size,
            brow_middle_color,
        );
        brow_right_middle.set_gap(5.0 + data.brow_length, &eye_right);

        let brow_right_outer = Ball::new_v(
            "right outer brow".into(),
            eye_right.clone() + Vector::new(0.0, -10.0, data.brow_length),
            data.brow_size,
            brow_outer_color,
        );
        brow_right_outer.set_gap(5.0 - mood_delta, &eye_right);

        let legs = Legs::new(&data, &butt, &shoulder);

        let mane = Mane::new(&data, &head, &shoulder);

        let tail_start_color = Color::hsl(data.hair_hue, data.hair_sat, 80);
        let tail_start = Ball::new_v(
            "tail start".into(),
            butt.clone() + Vector::new(10.0, -10.0, 0.0),
            data.tail_start_size,
            tail_start_color,
        );
        tail_start.move_to_sphere(&butt);

        let tail_end_color = Color::hsl(data.hair_hue, data.hair_sat, 60);
        let tail_end = Ball::new_v(
            "tail end".into(),
            tail_start.clone() + Vector::new(10.0, 0.0, 0.0),
            data.tail_end_size,
            tail_end_color,
        );
        tail_end.set_distance(data.tail_length, &tail_start);
        tail_end.rotate_around(*tail_start.center.borrow(), data.tail_angle, Axis::Z);

        let tail = Bone::non_linear_y(tail_start, tail_end, Gamma::new(data.tail_gamma, 0.3));

        let eye_curve = Gamma::new(1.5, 1.0);

        let face = Bone::new(snout, head);
        let horn = Bone::new(horn_onset, horn_tip);

        let brow_left_i =
            Bone::non_linear_y(brow_left_inner, brow_left_middle.clone(), eye_curve.clone());

        let brow_left_o = Bone::non_linear_y(brow_left_middle, brow_left_outer, eye_curve.clone());

        let brow_right_i = Bone::non_linear_y(
            brow_right_inner,
            brow_right_middle.clone(),
            eye_curve.clone(),
        );

        let brow_right_o = Bone::non_linear_y(brow_right_middle, brow_right_outer, eye_curve);

        let head = Head::new(
            face,
            horn,
            eye_left,
            eye_right,
            pupil_left,
            pupil_right,
            brow_left_i,
            brow_left_o,
            brow_right_i,
            brow_right_o,
        );

        let pivot = head.attachment().center.borrow().clone();
        head.rotate_around(pivot, data.face_tilt, Axis::X);

        let neck = Bone::new(head.attachment(), shoulder.clone());
        let neck = Neck::new(head.clone(), neck, mane);

        let pivot = head.attachment().center.borrow().clone();
        neck.rotate_around(pivot, data.face_tilt, Axis::X);

        let torso = Bone::new(shoulder.clone(), butt);
        let torso = Torso::new(neck, torso, tail, legs.clone());

        match data.pose {
            Pose::Walk { phase: _ } => {
                let low_front = if legs.fl.hoof.center.borrow().y > legs.fr.hoof.center.borrow().y {
                    legs.fl.hoof.center
                } else {
                    legs.fr.hoof.center
                };

                let low_back = if legs.bl.hoof.center.borrow().y > legs.br.hoof.center.borrow().y {
                    legs.bl.hoof.center
                } else {
                    legs.br.hoof.center
                };

                let angle = ((low_back.borrow().y - low_front.borrow().y)
                    / (low_back.borrow().x - low_front.borrow().x))
                    .atan();

                torso.rotate_around(*shoulder.center.borrow(), -angle, Axis::Z);
            }
            Pose::RotaryGallop { phase: _ } => {}
        }

        if data.x_angle < 0.0 {
            let pivot = shoulder.center.borrow().clone();
            torso.rotate_around(pivot.clone(), data.y_angle, Axis::Y);
            torso.rotate_around(pivot.clone(), data.x_angle, Axis::X);
            torso.rotate_around(pivot, -data.y_angle, Axis::Y);
        }

        Unicorn { torso }
    }

    pub fn head(&self) -> Ball {
        self.torso.neck.head.attachment()
    }

    pub fn shoulder(&self) -> Ball {
        self.torso.torso.b1.clone()
    }

    pub fn tracer(&self, world_view: WorldView) -> GroupTracer {
        let mut tracer = GroupTracer::new();

        self.torso.add_traceable(&mut tracer, world_view);

        tracer
    }
}
