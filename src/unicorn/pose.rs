use crate::geometry::Axis;
use crate::geometry::DEGREE;
use crate::pyrand::Random;
use crate::unicorn::Leg;
use crate::unicorn::Legs;
use crate::Sorter;
use crate::TV;

#[derive(Debug, Clone)]
pub enum Pose {
    RotaryGallop { phase: f64 },
    Walk { phase: f64 },
}

impl Pose {
    pub fn new(rand: &mut Random) -> Pose {
        let kind = rand.choice(2);
        let phase = rand.rand();

        match kind {
            0 => Pose::RotaryGallop { phase },
            1 => Pose::Walk { phase },
            _ => unreachable!(),
        }
    }

    pub fn pose(&self, legs: Legs) -> Legs {
        match self {
            Pose::RotaryGallop { phase } => rotary_gallop(legs, *phase),
            Pose::Walk { phase } => walk(legs, *phase),
        }
    }
}

fn rotary_gallop(legs: Legs, phase: f64) -> Legs {
    let fl = legs.fl;
    let fr = legs.fr;
    let bl = legs.bl;
    let br = legs.br;

    let front_top = Sorter::new(vec![TV::new(9.0 / 12.0, 74.0), TV::new(2.5 / 12.0, -33.0)]);
    let front_bottom = Sorter::new(vec![
        TV::new(2.0 / 12.0, 0.0),
        TV::new(6.0 / 12.0, -107.0),
        TV::new(8.0 / 12.0, -90.0),
        TV::new(10.0 / 12.0, 0.0),
    ]);

    let back_top = Sorter::new(vec![
        TV::new(11.0 / 12.0, -53.0),
        TV::new(4.0 / 12.0, 0.0),
        TV::new(6.0 / 12.0, 0.0),
    ]);
    let back_bottom = Sorter::new(vec![
        TV::new(11.0 / 12.0, 0.0),
        TV::new(1.5 / 12.0, 90.0),
        TV::new(6.0 / 12.0, 30.0),
        TV::new(8.0 / 12.0, 50.0),
    ]);

    let hip_center = *fr.hip.center.borrow();
    let knee_center = *fr.knee.center.borrow();

    fr.knee
        .rotate_around(hip_center, front_top.interpolate(phase) * DEGREE, Axis::Z);
    fr.hoof
        .rotate_around(hip_center, front_top.interpolate(phase) * DEGREE, Axis::Z);
    fr.hoof.rotate_around(
        knee_center,
        front_bottom.interpolate(phase) * DEGREE,
        Axis::Z,
    );
    let fr = Leg::new(fr.hip, fr.knee, fr.hoof);

    let hip_center = *fl.hip.center.borrow();
    let knee_center = *fl.knee.center.borrow();
    fl.knee.rotate_around(
        hip_center,
        front_top.interpolate(phase - 0.25) * DEGREE,
        Axis::Z,
    );
    fl.hoof.rotate_around(
        hip_center,
        front_top.interpolate(phase - 0.25) * DEGREE,
        Axis::Z,
    );
    fl.hoof.rotate_around(
        knee_center,
        front_bottom.interpolate(phase - 0.25) * DEGREE,
        Axis::Z,
    );
    let fl = Leg::new(fl.hip, fl.knee, fl.hoof);

    let hip_center = *br.hip.center.borrow();
    let knee_center = *br.knee.center.borrow();
    br.knee
        .rotate_around(hip_center, back_top.interpolate(phase) * DEGREE, Axis::Z);
    br.hoof
        .rotate_around(hip_center, back_top.interpolate(phase) * DEGREE, Axis::Z);
    br.hoof.rotate_around(
        knee_center,
        back_bottom.interpolate(phase) * DEGREE,
        Axis::Z,
    );
    let br = Leg::new(br.hip, br.knee, br.hoof);

    let hip_center = *bl.hip.center.borrow();
    let knee_center = *bl.knee.center.borrow();
    bl.knee.rotate_around(
        hip_center,
        back_top.interpolate(phase - 0.167) * DEGREE,
        Axis::Z,
    );
    bl.hoof.rotate_around(
        hip_center,
        back_top.interpolate(phase - 0.167) * DEGREE,
        Axis::Z,
    );
    bl.hoof.rotate_around(
        knee_center,
        back_bottom.interpolate(phase - 0.167) * DEGREE,
        Axis::Z,
    );
    let bl = Leg::new(bl.hip, bl.knee, bl.hoof);

    Legs::new(fr, fl, br, bl)
}

fn walk(legs: Legs, phase: f64) -> Legs {
    let fl = legs.fl;
    let fr = legs.fr;
    let bl = legs.bl;
    let br = legs.br;

    let front_top = Sorter::new(vec![TV::new(6.5 / 9.0, 40.0), TV::new(2.5 / 9.0, -35.0)]);
    let front_bottom = Sorter::new(vec![
        TV::new(7.0 / 9.0, 0.0),
        TV::new(2.0 / 9.0, 0.0),
        TV::new(5.0 / 9.0, -70.0),
    ]);

    let back_top = Sorter::new(vec![
        TV::new(1.0 / 9.0, -53.0),
        TV::new(4.0 / 9.0, 0.0),
        TV::new(6.0 / 9.0, 0.0),
    ]);
    let back_bottom = Sorter::new(vec![TV::new(5.0 / 9.0, 40.0), TV::new(9.0 / 9.0, 10.0)]);

    let hip_center = *fr.hip.center.borrow();
    let knee_center = *fr.knee.center.borrow();
    fr.knee
        .rotate_around(hip_center, front_top.interpolate(phase) * DEGREE, Axis::Z);
    fr.hoof
        .rotate_around(hip_center, front_top.interpolate(phase) * DEGREE, Axis::Z);
    fr.hoof.rotate_around(
        knee_center,
        front_bottom.interpolate(phase) * DEGREE,
        Axis::Z,
    );
    let fr = Leg::new(fr.hip, fr.knee, fr.hoof);

    let hip_center = *fl.hip.center.borrow();
    let knee_center = *fl.knee.center.borrow();
    fl.knee.rotate_around(
        hip_center,
        front_top.interpolate(phase - 0.56) * DEGREE,
        Axis::Z,
    );
    fl.hoof.rotate_around(
        hip_center,
        front_top.interpolate(phase - 0.56) * DEGREE,
        Axis::Z,
    );
    fl.hoof.rotate_around(
        knee_center,
        front_bottom.interpolate(phase - 0.56) * DEGREE,
        Axis::Z,
    );
    let fl = Leg::new(fl.hip, fl.knee, fl.hoof);

    let hip_center = *br.hip.center.borrow();
    let knee_center = *br.knee.center.borrow();
    br.knee
        .rotate_around(hip_center, back_top.interpolate(phase) * DEGREE, Axis::Z);
    br.hoof
        .rotate_around(hip_center, back_top.interpolate(phase) * DEGREE, Axis::Z);
    br.hoof.rotate_around(
        knee_center,
        back_bottom.interpolate(phase) * DEGREE,
        Axis::Z,
    );
    let br = Leg::new(br.hip, br.knee, br.hoof);

    let hip_center = *bl.hip.center.borrow();
    let knee_center = *bl.knee.center.borrow();
    bl.knee.rotate_around(
        hip_center,
        back_top.interpolate(phase - 0.44) * DEGREE,
        Axis::Z,
    );
    bl.hoof.rotate_around(
        hip_center,
        back_top.interpolate(phase - 0.44) * DEGREE,
        Axis::Z,
    );
    bl.hoof.rotate_around(
        knee_center,
        back_bottom.interpolate(phase - 0.44) * DEGREE,
        Axis::Z,
    );
    let bl = Leg::new(bl.hip, bl.knee, bl.hoof);

    Legs::new(fr, fl, br, bl)
}
