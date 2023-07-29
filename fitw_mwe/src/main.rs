use std::collections::HashMap;

const INITIAL_DOGS: i32 = 4;

struct Dog {
    id: i32
}

impl Dog {
    pub fn new(id: i32) -> Dog {
        let result = Dog { id: id };

        return result;
    }

    pub fn get_id(&mut self) -> i32 {
        return self.id;
    }

    pub fn bark(&mut self) {
        println!("Ruff!");
    }
}

struct Kennel {
    next_id: i32,
    dogs: HashMap<i32, Dog>
}

impl Kennel {
    pub fn new() -> Kennel {
        let result = Kennel { next_id: 1, dogs: HashMap::new() };

        return result;
    }

    fn add_dog(&mut self) {
        let mut new_dog = Dog::new(self.next_id);

        self.next_id += 1;
        self.dogs.insert(new_dog.get_id(), new_dog);
    }

    fn let_the_dogs_out(&mut self) {
        let mut ids_released = Vec::new();

        for (id, dog) in self.dogs.iter_mut() {
            ids_released.push(*id);
            dog.bark();
        }

        for id in &ids_released {
            self.dogs.remove(id);
        }
    }

    pub fn run_sim(&mut self) {
        for _ in 0..INITIAL_DOGS {
            self.add_dog();
        }

        self.let_the_dogs_out();
    }
}

fn main() {
    let mut kennel = Kennel::new();

    kennel.run_sim();
}
