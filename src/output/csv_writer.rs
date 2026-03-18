use crate::physics::body::Body;
use std::fs::File;

pub struct CsvWriter {
    writer: csv::Writer<File>,
}

impl CsvWriter {
    pub fn new(path: &str) -> Self {
        let file = File::create(path).expect("Failed to create file");
        let mut writer = csv::Writer::from_writer(file);

        writer.write_record(&["step", "body", "x", "y", "z"]).unwrap();

        Self { writer }
    }

    pub fn write_step(&mut self, step: usize, bodies: &Vec<Body>) {
        for body in bodies {

            if step % 10 == 0 {
                self.writer.write_record(&[
                    step.to_string(),
                    body.name.clone(),
                    body.position.x.to_string(),
                    body.position.y.to_string(),
                    body.position.z.to_string(),
                ]).unwrap();
            }
        }
    }

    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
}