use crate::Scene;

impl Scene {
    pub fn run(&self) {
        loop {
            println!("Game loop running...");

            // Simulate a frame delay
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
