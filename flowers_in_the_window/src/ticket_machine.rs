/// This code defines a class which models a ticket machine.

/// Structure.
pub struct TicketMachine {
    next: i32
}

/// Implementation.
impl TicketMachine {
    pub fn new() -> TicketMachine {
        let result = TicketMachine { next: 1 };

        return result;
    }

    pub fn get_ticket(&mut self) -> i32 {
        let result = self.next;

        self.next += 1;

        return result;
    }
}
