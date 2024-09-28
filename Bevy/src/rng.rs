pub struct Random;

impl Random {
    pub fn with_max(max: f32) -> f32 {
        return rand::random::<f32>() * max;
    }

    pub fn between(min: f32, max: f32) -> f32 {
        let range = max - min;
        return rand::random::<f32>() * range + min;
    }
}
