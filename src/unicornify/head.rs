use crate::unicornify::Ball;
use crate::unicornify::Bone;
use crate::Axis;
use crate::Vector;

#[derive(Clone, Debug)]
pub struct Head {
    pub face: Bone,
    pub horn: Bone,
    pub eye_left: Ball,
    pub eye_right: Ball,
    pub pupil_left: Ball,
    pub pupil_right: Ball,
    pub brow_left_i: Bone,
    pub brow_left_o: Bone,
    pub brow_right_i: Bone,
    pub brow_right_o: Bone,
}

impl Head {
    pub fn new(
        face: Bone,
        horn: Bone,
        eye_left: Ball,
        eye_right: Ball,
        pupil_left: Ball,
        pupil_right: Ball,
        brow_left_i: Bone,
        brow_left_o: Bone,
        brow_right_i: Bone,
        brow_right_o: Bone,
    ) -> Head {
        Head {
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
        }
    }

    pub fn attachment(&self) -> Ball {
        self.face.b2
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        self.face.rotate_around(other, angle, axis);
        self.horn.rotate_around(other, angle, axis);
        self.eye_left.rotate_around(other, angle, axis);
        self.eye_right.rotate_around(other, angle, axis);
        self.pupil_left.rotate_around(other, angle, axis);
        self.pupil_right.rotate_around(other, angle, axis);
        self.brow_left_i.rotate_around(other, angle, axis);
        self.brow_left_o.rotate_around(other, angle, axis);
        self.brow_right_i.rotate_around(other, angle, axis);
        self.brow_right_o.rotate_around(other, angle, axis);
    }
}
