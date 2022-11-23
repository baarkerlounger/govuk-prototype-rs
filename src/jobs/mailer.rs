use fang::Error;
use fang::Runnable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Job {
    pub number: u16,
}

#[typetag::serde]
impl Runnable for Job {
    fn run(&self) -> Result<(), Error> {
        println!("the number is {}", self.number);

        Ok(())
    }
}
