use super::{Human};

pub struct HumanController;

impl HumanController {

    pub fn get(human: &Human) -> String{
        format!(
            "Human: Age: {}, Stats: {:?}",
            human.age, human.stats.as_array()
        )
    }

}   
    