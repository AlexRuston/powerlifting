#[derive(Debug)]
pub struct Lifter {
    bodyweight: f32,
    squat: f32,
    bench: f32,
    deadlift: f32,
}

impl Lifter {
    // &self - an instance method
    pub fn total(&self) -> f32 {
        self.squat + self.bench + self.deadlift
    }

    pub fn wilks(&self) -> f32 {
        self.total() / self.bodyweight
    }

    pub fn new(bodyweight: f32, squat: f32, bench: f32, deadlift: f32) -> Lifter {
        Lifter {
            bodyweight: bodyweight,
            squat: squat,
            bench: bench,
            deadlift: deadlift,
        }
    }
}