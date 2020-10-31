use crate::Random;

use crate::unicornify::Pose;
use crate::unicornify::DEGREE;

pub struct Unicorn {
    head_size: f64,
    snout_size: f64,
    shoulder_size: f64,
    snout_length: f64,
    butt_size: f64,
    body_hue: i32,
    body_sat: i32,
    horn_hue: i32,
    horn_sat: i32,
    horn_onset_size: f64,
    horn_tip_size: f64,
    horn_length: f64,
    horn_angle: f64, // 0 means straight in x-direction, >0 means upwards
    eye_size: f64,
    iris_size: f64, // no longer used
    iris_hue: i32,  // no longer used
    iris_sat: i32,  // no longer used
    pupil_size: f64,
    hair_hue: i32,
    hair_sat: i32,
    hair_count: usize,
    hair_starts: Vec<f64>,
    hair_gammas: Vec<f64>,
    hair_lengths: Vec<f64>,
    hair_angles: Vec<f64>,
    hair_straightnesses: Vec<f64>, // for lack of a better word -- this is just the z offsets of the tip
    hair_tip_lightnesses: Vec<i32>,
    tail_start_size: f64,
    tail_end_size: f64,
    tail_length: f64,
    tail_angle: f64,
    tail_gamma: f64,
    brow_size: f64,
    brow_length: f64,
    brow_mood: f64, // from -1 (angry) to 1 (astonished)

    pose: Pose,

    neck_tilt: f64,
    face_tilt: f64,
}

impl Unicorn {
    pub fn new() -> Self {
        Unicorn {
            head_size: 0.0,
            snout_size: 0.0,
            shoulder_size: 0.0,
            snout_length: 0.0,
            butt_size: 0.0,
            body_hue: 0,
            body_sat: 0,
            horn_hue: 0,
            horn_sat: 0,
            horn_onset_size: 0.0,
            horn_tip_size: 0.0,
            horn_length: 0.0,
            horn_angle: 0.0,
            eye_size: 0.0,
            iris_size: 0.0,
            iris_hue: 0,
            iris_sat: 0,
            pupil_size: 0.0,
            hair_hue: 0,
            hair_sat: 0,
            hair_count: 0,
            hair_starts: Vec::new(),
            hair_gammas: Vec::new(),
            hair_lengths: Vec::new(),
            hair_angles: Vec::new(),
            hair_straightnesses: Vec::new(),
            hair_tip_lightnesses: Vec::new(),
            tail_start_size: 0.0,
            tail_end_size: 0.0,
            tail_length: 0.0,
            tail_angle: 0.0,
            tail_gamma: 0.0,
            brow_size: 0.0,
            brow_length: 0.0,
            brow_mood: 0.0,

            pose: Pose::RotaryGallop { phase: 0.0 },

            neck_tilt: 0.0,
            face_tilt: 0.0,
        }
    }

    pub fn rand1(&mut self, rand: &mut Random) {
        self.body_hue = rand.rand_i32(0, 359);
        self.body_sat = rand.rand_i32(50, 100);
        self.horn_hue = (self.body_hue + rand.rand_i32(60, 300)) % 360;
        self.horn_sat = rand.rand_i32(50, 100);
        self.snout_size = rand.rand_i32(8, 30) as f64;
        self.snout_length = rand.rand_i32(70, 110) as f64;
        self.head_size = rand.rand_i32(25, 40) as f64;
        self.shoulder_size = rand.rand_i32(40, 60) as f64;
        self.butt_size = rand.rand_i32(30, 60) as f64;
        self.horn_onset_size = rand.rand_i32(6, 12) as f64;
        self.horn_tip_size = rand.rand_i32(3, 6) as f64;
        self.horn_length = rand.rand_i32(50, 100) as f64;
        self.horn_angle = rand.rand_i32(10, 60) as f64 * DEGREE;
        self.eye_size = rand.rand_i32(8, 12) as f64;
        self.iris_size = rand.rand_i32(3, 6) as f64;
        self.iris_hue = rand.rand_i32(70, 270);
        self.iris_sat = rand.rand_i32(40, 70);
        self.pupil_size = rand.rand_i32(2, 5) as f64;
        let _ = rand.rand_i32(0, 60);
        self.hair_hue = (self.body_hue + rand.rand_i32(60, 300)) % 360;
        self.hair_sat = rand.rand_i32(60, 100);
        self.hair_count = (rand.rand_i32(12, 30) * 2) as usize;
        self.hair_starts = Vec::with_capacity(self.hair_count);
        self.hair_gammas = Vec::with_capacity(self.hair_count);
        self.hair_lengths = Vec::with_capacity(self.hair_count);
        self.hair_angles = Vec::with_capacity(self.hair_count);
        self.hair_tip_lightnesses = Vec::with_capacity(self.hair_count);
        self.hair_straightnesses = Vec::with_capacity(self.hair_count);

        self.make_hair1(rand, 0, self.hair_count / 2);
    }

    pub fn rand2(&mut self, rand: &mut Random) {
        self.make_hair2(rand, 0, self.hair_count / 2);

        self.tail_start_size = rand.rand_i32(4, 10) as f64;
        self.tail_end_size = rand.rand_i32(10, 20) as f64;
        self.tail_length = rand.rand_i32(100, 150) as f64;
        self.tail_angle = rand.rand_i32(-20, 45) as f64 * DEGREE;
        self.tail_gamma = 0.1 + rand.rand() * 6.0;
        self.brow_size = rand.rand_i32(2, 4) as f64;
        self.brow_length = 2.0 + rand.rand() * 3.0;
        self.brow_mood = 2.0 * rand.rand() - 1.0;

        let neck_tilt = rand.rand_i32(-30, 30);
        self.neck_tilt = neck_tilt as f64 * DEGREE;

        let a = neck_tilt / 3;
        let b = neck_tilt / 4;

        let face_tilt = if a > b {
            rand.rand_i32(b, a)
        } else {
            rand.rand_i32(a, b)
        };
        self.face_tilt = face_tilt as f64 * DEGREE;
    }

    pub fn rand3(&mut self, rand: &mut Random) {
        self.pose = Pose::new(rand);
    }

    pub fn rand4(&mut self, rand: &mut Random) {
        let half_count = self.hair_count / 2;

        self.make_hair1(rand, half_count, half_count);
        self.make_hair2(rand, half_count, half_count);
    }

    pub fn make_hair1(&mut self, rand: &mut Random, start: usize, count: usize) {
        for i in start..start + count {
            self.hair_starts[i] = rand.rand_i32(-20, 100) as f64;
        }

        for i in start..start + count {
            self.hair_gammas[i] = 0.3 + rand.rand() * 3.0;
        }

        for i in start..start + count {
            self.hair_lengths[i] = rand.rand_i32(80, 150) as f64;
        }

        for i in start..start + count {
            self.hair_angles[i] = (rand.rand_i32(0, 60) as f64) * DEGREE;
        }
    }

    pub fn make_hair2(&mut self, rand: &mut Random, start: usize, count: usize) {
        for i in start..start + count {
            self.hair_tip_lightnesses[i] = rand.rand_i32(40, 85);
        }

        for i in start..start + count {
            self.hair_straightnesses[i] = rand.rand_i32(-40, 40) as f64;
        }
    }
}
