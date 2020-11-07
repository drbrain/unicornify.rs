use crate::Axis;
use crate::Color;
use crate::Gamma;
use crate::Vector;

use crate::unicornify::Ball;
use crate::unicornify::Pose;
use crate::unicornify::Bone;
use crate::unicornify::Figure;
use crate::unicornify::Leg;
use crate::unicornify::Legs;
use crate::unicornify::Thing;
use crate::unicornify::UnicornData;

#[derive(Debug)]
pub struct Unicorn {
    figure: Figure,
}

impl Unicorn {
    pub fn new(data: UnicornData) -> Self {
        let head_color = Color::hsl(data.body_hue, data.body_sat, 60);
        let head = Ball::new(0.0, 0.0, 0.0, data.head_size, head_color);

        let snout_color = Color::hsl(data.body_hue, data.body_sat, 80);
        let snout = Ball::new(-25.0, 60.0, 0.0, data.snout_size, snout_color);
        let snout = snout.set_distance(data.snout_length, head.clone());

        let shoulder_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let shoulder = Ball::new(80.0, 120.0, 0.0, data.shoulder_size, shoulder_color);

        let butt_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let butt = Ball::new(235.0, 155.0, 0.0, data.butt_size, butt_color);

        let horn_root_color = Color::hsl(data.horn_hue, data.horn_sat, 70);
        let horn_onset = Ball::new(-22.0, -10.0, 0.0, data.horn_onset_size, horn_root_color);
        let horn_onset = horn_onset.move_to_sphere(head.clone());

        let horn_tip_color = Color::hsl(data.horn_hue, data.horn_sat, 90);
        let tip_pos = horn_onset.center + Vector::new(-10.0, 0.0, 0.0);
        let horn_tip = Ball::new(
            tip_pos.x,
            tip_pos.y,
            tip_pos.z,
            data.horn_tip_size,
            horn_tip_color,
        );
        let horn_tip = horn_tip.set_distance(data.horn_length, horn_onset.clone());
        let horn_tip = horn_tip.rotate_around(horn_onset.center.clone(), data.horn_angle, Axis::Z);

        let eye_left = Ball::new(-10.0, 3.0, -5.0, data.eye_size, Color::white());
        let eye_left = eye_left.set_gap(5.0, head.clone());

        let eye_right = Ball::new(-10.0, 3.0, 5.0, data.eye_size, Color::white());
        let eye_right = eye_right.set_gap(5.0, head.clone());

        let pupil_left = Ball::new_v(
            eye_left.center + Vector::new(-1.0, 0.0, 0.0),
            data.pupil_size,
            Color::black(),
        );
        let pupil_left = pupil_left.move_to_sphere(eye_left.clone());

        let pupil_right = Ball::new_v(
            eye_right.center + Vector::new(-1.0, 0.0, 0.0),
            data.pupil_size,
            Color::black(),
        );
        let pupil_right = pupil_right.move_to_sphere(eye_right.clone());

        let mood_delta = data.brow_mood * 3.0;

        let brow_inner_color = Color::hsl(data.hair_hue, data.hair_sat, 50);
        let brow_middle_color = Color::hsl(data.hair_hue, data.hair_sat, 70);
        let brow_outer_color = Color::hsl(data.hair_hue, data.hair_sat, 60);

        let brow_left_inner = Ball::new_v(
            eye_left.center + Vector::new(0.0, -10.0, data.brow_length),
            data.brow_size,
            brow_inner_color.clone(),
        );
        let brow_left_inner = brow_left_inner.set_gap(5.0 + mood_delta, eye_left.clone());

        let brow_left_middle = Ball::new_v(
            eye_left.center + Vector::new(0.0, -10.0, 0.0),
            data.brow_size,
            brow_middle_color.clone(),
        );
        let brow_left_middle = brow_left_middle.set_gap(5.0 + mood_delta, eye_left.clone());

        let brow_left_outer = Ball::new_v(
            eye_left.center + Vector::new(0.0, -10.0, -data.brow_length),
            data.brow_size,
            brow_outer_color.clone(),
        );
        let brow_left_outer = brow_left_outer.set_gap(5.0 - mood_delta, eye_left.clone());

        let brow_right_inner = Ball::new_v(
            eye_right.center + Vector::new(0.0, -10.0, -data.brow_length),
            data.brow_size,
            brow_inner_color.clone(),
        );
        let brow_right_inner = brow_right_inner.set_gap(5.0 + mood_delta, eye_right.clone());

        let brow_right_middle = Ball::new_v(
            eye_right.center + Vector::new(0.0, -10.0, 0.0),
            data.brow_size,
            brow_middle_color,
        );
        let brow_right_middle = brow_right_middle.set_gap(5.0 + mood_delta, eye_right.clone());

        let brow_right_outer = Ball::new_v(
            eye_right.center + Vector::new(0.0, -10.0, data.brow_length),
            data.brow_size,
            brow_outer_color,
        );
        let brow_right_outer = brow_right_outer.set_gap(5.0 - mood_delta, eye_right.clone());

        let hip_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let knee_color = Color::hsl(data.body_hue, data.body_sat, 70);
        let hoof_color = Color::hsl(data.body_hue, data.body_sat, 45);

        let mut legs: Vec<Leg> = Vec::with_capacity(4);
        let offsets: [f64; 2] = [-25.0, 25.0];

        for z in offsets.iter() {
            let hip = Ball::new(55.0, 160.0, *z, 25.0, hip_color.clone());
            let knee = Ball::new(35.0, 254.0, *z, 9.0, knee_color.clone());
            let hoof = Ball::new(-55.0, 310.0, *z, 11.0, hoof_color.clone());
            let hip = hip.move_to_sphere(shoulder.clone());

            let leg = Leg::new(hip, knee, hoof);
            legs.push(leg);
        }

        for z in offsets.iter() {
            let hip = Ball::new(255.0, 190.0, *z, 25.0, hip_color.clone());
            let knee = Ball::new(230.0, 264.0, *z, 9.0, knee_color.clone());
            let hoof = Ball::new(220.0, 310.0, *z, 11.0, hoof_color.clone());
            let hip = hip.move_to_sphere(butt.clone());

            let leg = Leg::new(hip, knee, hoof);
            legs.push(leg);
        }

        let legs = Legs::new(legs.remove(0), legs.remove(0), legs.remove(0), legs.remove(0));
        let legs = data.pose.pose(legs);

        let hairs = hairs(&data, head.clone(), shoulder.clone());

        let tail_start_color = Color::hsl(data.hair_hue, data.hair_sat, 80);
        let tail_start = Ball::new_v(
            butt.center + Vector::new(10.0, -10.0, 0.0),
            data.tail_start_size,
            tail_start_color,
        );
        let tail_start = tail_start.move_to_sphere(butt.clone());

        let tail_end_color = Color::hsl(data.hair_hue, data.hair_sat, 60);
        let tail_end = Ball::new_v(
            tail_start.center + Vector::new(10.0, 0.0, 0.0),
            data.tail_end_size,
            tail_end_color,
        );
        let tail_end = tail_end.set_distance(data.tail_length, tail_start.clone());
        let tail_end = tail_end.rotate_around(tail_start.center.clone(), data.tail_angle, Axis::Z);
        let tail = Bone::non_linear_y(tail_start, tail_end, Gamma::new(data.tail_gamma, 0.3));

        let eye_curve = Gamma::new(1.5, 1.0);

        let mut figure = Figure::new();
        figure.push(Thing::BoneT(Bone::new(snout, head.clone())));
        figure.push(Thing::BoneT(Bone::new(horn_onset, horn_tip)));
        figure.push(Thing::BallT(eye_left));
        figure.push(Thing::BallT(eye_right));
        figure.push(Thing::BallT(pupil_left));
        figure.push(Thing::BallT(pupil_right));

        figure.push(Thing::BoneT(Bone::non_linear_y(
            brow_left_inner,
            brow_left_middle.clone(),
            eye_curve.clone(),
        )));

        figure.push(Thing::BoneT(Bone::non_linear_y(
            brow_left_middle,
            brow_left_outer,
            eye_curve.clone(),
        )));

        figure.push(Thing::BoneT(Bone::non_linear_y(
            brow_right_inner,
            brow_right_middle.clone(),
            eye_curve.clone(),
        )));

        figure.push(Thing::BoneT(Bone::non_linear_y(
            brow_right_middle,
            brow_right_outer,
            eye_curve,
        )));

        let head_rotation_v = head.clone().center;

        for t in figure.iter() {
            t.rotate_around(head_rotation_v, data.face_tilt, Axis::X);
        }

        figure.push(Thing::BoneT(Bone::new(head.clone(), shoulder.clone())));
        figure.push(Thing::FigureT(hairs));

        for t in figure.iter() {
            t.rotate_around(head_rotation_v, data.face_tilt, Axis::X);
        }

        figure.push(Thing::BoneT(Bone::new(shoulder.clone(), butt)));
        figure.push(Thing::BoneT(tail.clone()));

        figure.push(Thing::BoneT(legs.fr.calf));
        figure.push(Thing::BoneT(legs.fr.shin));
        figure.push(Thing::BoneT(legs.fl.calf));
        figure.push(Thing::BoneT(legs.fl.shin));
        figure.push(Thing::BoneT(legs.br.calf));
        figure.push(Thing::BoneT(legs.br.shin));
        figure.push(Thing::BoneT(legs.bl.calf));
        figure.push(Thing::BoneT(legs.bl.shin));

        match data.pose {
            Pose::Walk { phase: _ } => {
                let low_front = if legs.fl.hoof.center.y > legs.fr.hoof.center.y {
                    legs.fl.hoof.center
                } else {
                    legs.fr.hoof.center
                };

                let low_back = if legs.bl.hoof.center.y > legs.br.hoof.center.y {
                    legs.bl.hoof.center
                } else {
                    legs.br.hoof.center
                };

                let angle = ((low_back.y - low_front.y) / (low_back.x - low_front.x)).atan();

                for t in figure.iter() {
                    t.rotate_around(shoulder.center, -angle, Axis::Z);
                }
            },
            Pose::RotaryGallop { phase: _ } => {},
        }

        if data.x_angle < 0.0 {
            for t in figure.iter() {
                t.rotate_around(shoulder.center, data.y_angle, Axis::Y);
                t.rotate_around(shoulder.center, data.x_angle, Axis::X);
                t.rotate_around(shoulder.center, -data.y_angle, Axis::Y);
            }
        }

        Unicorn { figure }
    }
}

fn hairs(data: &UnicornData, head: Ball, shoulder: Ball) -> Figure {
    let hair_top = Ball::new_v(
        head.center + Vector::new(10.0, -5.0, 0.0),
        5.0,
        Color::white(),
    );
    let hair_top = hair_top.move_to_sphere(head.clone());
    let hair_bottom = Ball::new_v(
        shoulder.center + Vector::new(10.0, -15.0, 0.0),
        5.0,
        Color::white(),
    );
    let hair_bottom = hair_bottom.move_to_sphere(shoulder.clone());

    let hair_span = hair_bottom.center - hair_top.center;

    let mut hairs = Figure::new();

    let hair_color = Color::hsl(data.hair_hue, data.hair_sat, 60);

    for i in 0..data.hair_starts.len() {
        let start = hair_top.center + hair_span * data.hair_starts[i] / 100.0;
        let hair_start = Ball::new_v(start, 5.0, hair_color.clone());

        let end = Vector::new(
            start.x + data.hair_lengths[i],
            start.y,
            start.z + data.hair_straightnesses[i],
        );
        let end_color = Color::hsl(data.hair_hue, data.hair_sat, data.hair_tip_lightnesses[i]);
        let hair_end = Ball::new_v(end, 2.0, end_color);

        let hair = Bone::non_linear(
            hair_start,
            hair_end,
            Gamma::new(data.hair_gammas[i], 0.2),
            Gamma::new(1.0 / data.hair_gammas[i], 0.2),
        );

        hairs.push(Thing::BoneT(hair));
    }

    hairs
}
