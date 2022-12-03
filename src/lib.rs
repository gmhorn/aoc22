use std::time::Instant;

pub struct Timer(Instant);

impl Timer {
    pub fn tick() -> Self {
        Self(Instant::now())
    }

    pub fn tock(&self) {
        println!("{} ms", (Instant::now() - self.0).as_millis())
    }
}
