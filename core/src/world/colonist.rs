use crate::data::name_gen;
use crate::data::names::{FirstName, LastName};

const MIN_AGE: u8 = 18;
const MAX_AGE: u8 = 80;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Sex {
    Female,
    Male,
}

impl Sex {
    pub fn random(rng: &mut fastrand::Rng) -> Self {
        if rng.bool() { Sex::Female } else { Sex::Male }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Colonist {
    pub sex: Sex,
    pub age: u8,
    pub first_name: FirstName,
    pub last_name: LastName,
}

impl Colonist {
    pub fn generate(rng: &mut fastrand::Rng) -> Self {
        let sex = Sex::random(rng);
        let (first_name, last_name) = name_gen::generate(rng, sex);
        Self {
            sex,
            age: rng.u8(MIN_AGE..=MAX_AGE),
            first_name,
            last_name,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name.as_str(), self.last_name.as_str())
    }
}
