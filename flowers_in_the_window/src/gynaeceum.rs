/// This code defines a class which models "the Gynaeceum".

// Standard imports.
use std::collections::HashMap;

// Non-standard imports.
use rand::seq::SliceRandom;

// Local imports.
use crate::female;
use crate::ticket_machine;

// Local constants.
const SIM_LENGTH_IN_DAYS: i32 = 365*10;
const SIM_INITIAL_FEMALE_COUNT: i32 = 4;

/// Structure.
pub struct Gynaeceum {
    duration_in_days: i32,
    ticket_machine: ticket_machine::TicketMachine,
    females: HashMap<i32, female::Female>
}

/// Implementation.
impl Gynaeceum {
    pub fn new() -> Gynaeceum {
        let result = 
            Gynaeceum {
                duration_in_days: 0,
                ticket_machine: ticket_machine::TicketMachine::new(),
                females: HashMap::new()
            };

        return result;
    }

    fn add_female(&mut self) {
        let mut new_female =
            female::Female::new(self.ticket_machine.get_ticket());

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
        let selected = self.females.entry(&selected_id);

        selected.copulate();

        return true;
    }

    fn tick(&mut self) {
        let available = Vec::new();
        let retiree_ids = Vec::new();

        for (id, female) in &self.females {
            let status_code = female.tick();

            if status_code == female::FemaleStatusCode::GivenBirth {
                if female.give_birth() {
                    self.add_female();
                }
            } else if status_code == female::FemaleStatusCode::Infertile {
                retiree_ids.push(*id);
            } else if status_code == female::FemaleStatusCode::Normal {
                available.push(*id);
            }

            self.process_retirees(retiree_ids);
            self.service(available);
        }
    }

    pub fn run_sim(&mut self) {
        for _ in 0..SIM_INITIAL_FEMALE_COUNT {
            self.add_female();
        }

        for _ in 0..SIM_LENGTH_IN_DAYS {
            self.tick();
        }

        println!("Female count: {}", self.females.len());
    }
}
