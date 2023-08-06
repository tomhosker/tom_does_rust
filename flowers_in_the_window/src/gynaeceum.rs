/// This code defines a class which models "the Gynaeceum".

// Standard imports.
use std::collections::HashMap;

// Non-standard imports.
use rand::seq::SliceRandom;

// Local imports.
use crate::female;

// Local constants.
const SIM_LENGTH_IN_DAYS: i32 = 365*1000;
const SIM_INITIAL_FEMALE_COUNT: i32 = 4;
const FOUNDING_FEMALE_AGE_IN_DAYS: i32 = 365*20;

/// Structure.
pub struct Gynaeceum {
    duration_in_days: i32,
    next_ticket: i32,
    females: HashMap<i32, female::Female>
}

/// Implementation.
impl Gynaeceum {
    pub fn new() -> Gynaeceum {
        let result = 
            Gynaeceum {
                duration_in_days: 0,
                next_ticket: 1,
                females: HashMap::new()
            };

        return result;
    }

    fn get_ticket(&mut self) -> i32 {
        let result = self.next_ticket;

        self.next_ticket += 1;

        return result;
    }

    fn add_female(&mut self) {
        let mut new_female = female::Female::new(self.get_ticket());

        self.females.insert(new_female.get_id(), new_female);
    }

    fn process_retirees(&mut self, retiree_ids: Vec<i32>) {
        for id in retiree_ids {
            self.females.remove(&id);
        }
    }

    fn service(&mut self, available: Vec<i32>) -> bool {
        if available.len() == 0 {
            return false;
        }

        let selected_id = available.choose(&mut rand::thread_rng());
        let selected = self.females.get_mut(selected_id.unwrap()).unwrap();

        selected.copulate();

        return true;
    }

    fn tick(&mut self) {
        let mut available = Vec::new();
        let mut retiree_ids = Vec::new();
        let mut birth_count = 0;

        for (id, female) in self.females.iter_mut() {
            let status_code = female.tick();

            if status_code == female::FemaleStatusCode::GivenBirth {
                if female.give_birth() {
                    birth_count += 1;
                }
            } else if status_code == female::FemaleStatusCode::Infertile {
                retiree_ids.push(*id);
            } else if status_code == female::FemaleStatusCode::Normal {
                if female.can_copulate() {
                    available.push(*id);
                }
            }
        }

        for _ in 0..birth_count {
            self.add_female();
        }

        self.process_retirees(retiree_ids);
        self.service(available);
        self.duration_in_days += 1;
    }

    pub fn run_sim(&mut self) {
        for _ in 0..SIM_INITIAL_FEMALE_COUNT {
            self.add_female();
        }

        for (_, female) in self.females.iter_mut() {
            female.set_age(FOUNDING_FEMALE_AGE_IN_DAYS);
        }

        while self.duration_in_days <= SIM_LENGTH_IN_DAYS {
            self.tick();
        }

        println!("Female count: {}", self.females.len());
    }
}
