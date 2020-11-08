use crate::Axis;
use crate::Color;
use crate::Gamma;
use crate::Vector;

use crate::unicornify::Ball;
use crate::unicornify::Bone;
use crate::unicornify::Head;
use crate::unicornify::Leg;
use crate::unicornify::Legs;
use crate::unicornify::Mane;
use crate::unicornify::Neck;
use crate::unicornify::Pose;
use crate::unicornify::Torso;
use crate::unicornify::UnicornData;

#[derive(Debug)]
pub struct Unicorn {
    torso: Torso,
}

impl Unicorn {
    pub fn new(data: UnicornData) -> Self {
        let head_color = Color::hsl(data.body_hue, data.body_sat, 60);
        let head = Ball::new(0.0, 0.0, 0.0, data.head_size, head_color);

        let snout_color = Color::hsl(data.body_hue, data.body_sat, 80);
        let snout = Ball::new(-25.0, 60.0, 0.0, data.snout_size, snout_color);
        snout.set_distance(data.snout_length, head.clone());

        let shoulder_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let shoulder = Ball::new(80.0, 120.0, 0.0, data.shoulder_size, shoulder_color);

        let butt_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let butt = Ball::new(235.0, 155.0, 0.0, data.butt_size, butt_color);

        let horn_root_color = Color::hsl(data.horn_hue, data.horn_sat, 70);
        let horn_onset = Ball::new(-22.0, -10.0, 0.0, data.horn_onset_size, horn_root_color);
        horn_onset.move_to_sphere(head.clone());

        let horn_tip_color = Color::hsl(data.horn_hue, data.horn_sat, 90);
        let tip_pos = horn_onset + Vector::new(-10.0, 0.0, 0.0);
        let horn_tip = Ball::new(
            tip_pos.x,
            tip_pos.y,
            tip_pos.z,
            data.horn_tip_size,
            horn_tip_color,
        );
        horn_tip.set_distance(data.horn_length, horn_onset.clone());
        horn_tip.rotate_around(*horn_onset.center.borrow(), data.horn_angle, Axis::Z);

        let eye_left = Ball::new(-10.0, 3.0, -5.0, data.eye_size, Color::white());
        eye_left.set_gap(5.0, head.clone());

        let eye_right = Ball::new(-10.0, 3.0, 5.0, data.eye_size, Color::white());
        eye_right.set_gap(5.0, head.clone());

        let pupil_left = Ball::new_v(
            eye_left + Vector::new(-1.0, 0.0, 0.0),
            data.pupil_size,
            Color::black(),
        );
        pupil_left.move_to_sphere(eye_left.clone());

        let pupil_right = Ball::new_v(
            eye_right + Vector::new(-1.0, 0.0, 0.0),
            data.pupil_size,
            Color::black(),
        );
        pupil_right.move_to_sphere(eye_right.clone());

        let mood_delta = data.brow_mood * 3.0;

        let brow_inner_color = Color::hsl(data.hair_hue, data.hair_sat, 50);
        let brow_middle_color = Color::hsl(data.hair_hue, data.hair_sat, 70);
        let brow_outer_color = Color::hsl(data.hair_hue, data.hair_sat, 60);

        let brow_left_inner = Ball::new_v(
            eye_left + Vector::new(0.0, -10.0, data.brow_length),
            data.brow_size,
            brow_inner_color.clone(),
        );
        brow_left_inner.set_gap(5.0 + mood_delta, eye_left.clone());

        let brow_left_middle = Ball::new_v(
            eye_left + Vector::new(0.0, -10.0, 0.0),
            data.brow_size,
            brow_middle_color.clone(),
        );
        brow_left_middle.set_gap(5.0 + mood_delta, eye_left.clone());

        let brow_left_outer = Ball::new_v(
            eye_left + Vector::new(0.0, -10.0, -data.brow_length),
            data.brow_size,
            brow_outer_color.clone(),
        );
        brow_left_outer.set_gap(5.0 - mood_delta, eye_left.clone());

        let brow_right_inner = Ball::new_v(
            eye_right + Vector::new(0.0, -10.0, -data.brow_length),
            data.brow_size,
            brow_inner_color.clone(),
        );
        brow_right_inner.set_gap(5.0 + mood_delta, eye_right.clone());

        let brow_right_middle = Ball::new_v(
            eye_right + Vector::new(0.0, -10.0, 0.0),
            data.brow_size,
            brow_middle_color,
        );
        brow_right_middle.set_gap(5.0 + mood_delta, eye_right.clone());

        let brow_right_outer = Ball::new_v(
            eye_right + Vector::new(0.0, -10.0, data.brow_length),
            data.brow_size,
            brow_outer_color,
        );
        brow_right_outer.set_gap(5.0 - mood_delta, eye_right.clone());

        let hip_color = Color::hsl(data.body_hue, data.body_sat, 40);
        let knee_color = Color::hsl(data.body_hue, data.body_sat, 70);
        let hoof_color = Color::hsl(data.body_hue, data.body_sat, 45);

        let mut legs: Vec<Leg> = Vec::with_capacity(4);
        let offsets: [f64; 2] = [-25.0, 25.0];

        for z in offsets.iter() {
            let hip = Ball::new(55.0, 160.0, *z, 25.0, hip_color.clone());
            let knee = Ball::new(35.0, 254.0, *z, 9.0, knee_color.clone());
            let hoof = Ball::new(-55.0, 310.0, *z, 11.0, hoof_color.clone());
            hip.move_to_sphere(shoulder.clone());

            let leg = Leg::new(hip, knee, hoof);
            legs.push(leg);
        }

        for z in offsets.iter() {
            let hip = Ball::new(255.0, 190.0, *z, 25.0, hip_color.clone());
            let knee = Ball::new(230.0, 264.0, *z, 9.0, knee_color.clone());
            let hoof = Ball::new(220.0, 310.0, *z, 11.0, hoof_color.clone());
            hip.move_to_sphere(butt.clone());

            let leg = Leg::new(hip, knee, hoof);
            legs.push(leg);
        }

        let legs = Legs::new(
            legs.remove(0),
            legs.remove(0),
            legs.remove(0),
            legs.remove(0),
        );
        let legs = data.pose.pose(legs);

        let mane = mane(&data, head.clone(), shoulder.clone());

        let tail_start_color = Color::hsl(data.hair_hue, data.hair_sat, 80);
        let tail_start = Ball::new_v(
            butt + Vector::new(10.0, -10.0, 0.0),
            data.tail_start_size,
            tail_start_color,
        );
        tail_start.move_to_sphere(butt.clone());

        let tail_end_color = Color::hsl(data.hair_hue, data.hair_sat, 60);
        let tail_end = Ball::new_v(
            tail_start + Vector::new(10.0, 0.0, 0.0),
            data.tail_end_size,
            tail_end_color,
        );
        tail_end.set_distance(data.tail_length, tail_start.clone());
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

        let head_rotation_v = head.attachment().center;

        head.rotate_around(*head.attachment().center.borrow(), data.face_tilt, Axis::X);

        let neck = Bone::new(head.attachment(), shoulder.clone());
        let neck = Neck::new(head, neck, mane);

        neck.rotate_around(*head_rotation_v.borrow(), data.face_tilt, Axis::X);

        let torso = Bone::new(shoulder.clone(), butt);
        let torso = Torso::new(neck, torso, tail, legs);

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
            torso.rotate_around(*shoulder.center.borrow(), data.y_angle, Axis::Y);
            torso.rotate_around(*shoulder.center.borrow(), data.x_angle, Axis::X);
            torso.rotate_around(*shoulder.center.borrow(), -data.y_angle, Axis::Y);
        }

        let fsize = size as f64;
        let factor = (unicorn_data.scale_factor - 0.5).sqrt() / 2.5;

        let look_at_point = shoulder + ((head.attachment() - shoulder) * factor);
        let camera_position = look_at_point + Vector::new(0.0, 0.0, -3.0 * focal_length);
        camera_position.rotate_around(
            *head.attachment().center.borrow(),
            -unicorn_data.x_angle,
            Axis::X,
        );
        camera_position.rotate_around(
            *head.attachment().center.borrow(),
            -unicorn_data.y_angle,
            Axis::Y,
        );

        Unicorn { torso }
    }
}

fn mane(data: &UnicornData, head: Ball, shoulder: Ball) -> Mane {
    let hair_top = Ball::new_v(head + Vector::new(10.0, -5.0, 0.0), 5.0, Color::white());
    hair_top.move_to_sphere(head.clone());
    let hair_bottom = Ball::new_v(
        shoulder + Vector::new(10.0, -15.0, 0.0),
        5.0,
        Color::white(),
    );
    hair_bottom.move_to_sphere(shoulder.clone());

    let hair_span = hair_bottom - hair_top;

    let mut mane = Mane::new(data.hair_starts.len());

    let hair_color = Color::hsl(data.hair_hue, data.hair_sat, 60);

    for i in 0..data.hair_starts.len() {
        let start = hair_top + hair_span * data.hair_starts[i] / 100.0;
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

        mane.push(hair);
    }

    mane
}
