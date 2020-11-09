use crate::geometry::Axis;
use crate::geometry::Ball;
use crate::geometry::Bone;
use crate::geometry::Gamma;
use crate::geometry::Vector;
use crate::Color;
use crate::Data;

#[derive(Clone, Debug)]
pub struct Mane {
    pub mane: Vec<Bone>,
}

impl Mane {
    pub fn new(data: &Data, head: Ball, shoulder: Ball) -> Self {
        let hair_top = Ball::new_v(
            "hair top".into(),
            head.clone() + Vector::new(10.0, -5.0, 0.0),
            5.0,
            Color::white(),
        );
        hair_top.move_to_sphere(head.clone());
        let hair_bottom = Ball::new_v(
            "hair bottom".into(),
            shoulder.clone() + Vector::new(10.0, -15.0, 0.0),
            5.0,
            Color::white(),
        );
        hair_bottom.move_to_sphere(shoulder.clone());

        let hair_span = hair_bottom - hair_top.clone();
        let hair_color = Color::hsl(data.hair_hue, data.hair_sat, 60);

        let mut mane: Vec<Bone> = Vec::with_capacity(data.hair_starts.len());

        for i in 0..data.hair_starts.len() {
            let start = hair_top.clone() + hair_span * data.hair_starts[i] / 100.0;
            let hair_start =
                Ball::new_v(format!("hair {} start", i), start, 5.0, hair_color.clone());

            let end = Vector::new(
                start.x + data.hair_lengths[i],
                start.y,
                start.z + data.hair_straightnesses[i],
            );
            let end_color = Color::hsl(data.hair_hue, data.hair_sat, data.hair_tip_lightnesses[i]);
            let hair_end = Ball::new_v(format!("hair {} end", i), end, 2.0, end_color);

            let hair = Bone::non_linear(
                hair_start,
                hair_end,
                Gamma::new(data.hair_gammas[i], 0.2),
                Gamma::new(1.0 / data.hair_gammas[i], 0.2),
            );

            mane.push(hair);
        }

        Mane { mane }
    }

    pub fn push(&mut self, hair: Bone) {
        self.mane.push(hair);
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        for hair in self.mane.iter() {
            hair.rotate_around(other, angle, axis);
        }
    }
}
