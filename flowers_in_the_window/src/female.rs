/// This code defines the Female class.

// Local imports.
use crate::utils;

// Local constants.
const PROB_FALLING_PREGNANT_PER_COPULATION: f64 = 0.05;
const PROB_FEMALE_OFFSPRING: f64 = 0.5;
const LENGTH_OF_PREGNANCY_IN_DAYS: i32 = 270;
const LENGTH_OF_YEAR_IN_DAYS: i32 = 365;
const MIN_FERTILE_AGE_IN_DAYS: i32 = 15*LENGTH_OF_YEAR_IN_DAYS;
const MAX_FERTILE_AGE_IN_DAYS: i32 = 45*LENGTH_OF_YEAR_IN_DAYS;

// Helper enum.
#[derive(PartialEq)]
pub enum FemaleStatusCode {
    Normal,
    GivenBirth,
    Infertile
}

/// Structure.
pub struct Female {
    id: i32,
    age_in_days: i32,
    is_pregnant: bool,
    days_of_pregnancy: i32
}

/// Implementation.
impl Female {
    pub fn new(id: i32) -> Female {
        let result =
            Female {
                id: id,
                age_in_days: 0,
                is_pregnant: false,
                days_of_pregnancy: 0
            };

        return result;
    }

    pub fn set_age(&mut self, new_age_in_days: i32) {
        self.age_in_days = new_age_in_days;
    }

    pub fn can_copulate(&mut self) -> bool {
        if
            self.is_pregnant ||
            self.age_in_days < MIN_FERTILE_AGE_IN_DAYS ||
            self.age_in_days > MAX_FERTILE_AGE_IN_DAYS
        {
            return false;
        }

        return true;
    }

    pub fn copulate(&mut self) -> bool {
        if self.is_pregnant {
            return false;
        }

        if utils::happens(PROB_FALLING_PREGNANT_PER_COPULATION) {
            self.is_pregnant = true;

            return true;
        }

        return false;
    }

    pub fn give_birth(&mut self) -> bool {
        self.is_pregnant = false;
        self.days_of_pregnancy = 0;

        if utils::happens(PROB_FEMALE_OFFSPRING) {
            return true; // I.e. has given birth to a girl.
        }

        return false; // I.e. has given birth to a boy.
    }

    pub fn tick(&mut self) -> FemaleStatusCode {
        self.age_in_days += 1;

        if self.is_pregnant {
            if self.days_of_pregnancy > LENGTH_OF_PREGNANCY_IN_DAYS {
                return FemaleStatusCode::GivenBirth;
            } else {
                self.days_of_pregnancy += 1;
            }
        } else if self.age_in_days > MAX_FERTILE_AGE_IN_DAYS {
            return FemaleStatusCode::Infertile;
        }

        return FemaleStatusCode::Normal;
    }

    pub fn get_id(&mut self) -> i32 {
        return self.id;
    }
}
