use std::path::PathBuf;
use std::fs;

pub struct TestSetup {
    pub test_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl TestSetup {
    pub fn new() -> Self {
        let test_dir = PathBuf::from("test_data");
        let output_dir = PathBuf::from("test_output");
        
        fs::create_dir_all(&test_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        Self {
            test_dir,
            output_dir,
        }
    }

    pub fn create_file(&self, name: &str, content: &str) {
        fs::write(self.test_dir.join(name), content).unwrap();
    }
}

impl Drop for TestSetup {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.test_dir).ok();
        fs::remove_dir_all(&self.output_dir).ok();
    }
}
