use std::cell::Cell;

pub struct AustroHungarianGreeter {
    counter: Cell<u32>,
}

impl AustroHungarianGreeter {
    pub fn new() -> Self {
        Self {
            counter: Cell::new(0),
        }
    }

    pub fn greet(&self) -> &'static str {
        let count = self.counter.get();
        self.counter.set(count + 1);
        match count % 3 {
            0 => "Es lebe der Kaiser!",
            1 => "Möge uns der Kaiser schützen!",
            2 => "Éljen Ferenc József császár!",
            _ => unreachable!(),
        }
    }
}

impl Drop for AustroHungarianGreeter {
    fn drop(&mut self) {
        println!("Ich habe {} mal gegrüßt", self.counter.get());
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::AustroHungarianGreeter;

    #[test]
    fn test_greeter() {
        let greeter = AustroHungarianGreeter {
            counter: Cell::new(0),
        };

        assert_eq!(greeter.greet(), "Es lebe der Kaiser!");
        assert_eq!(greeter.greet(), "Möge uns der Kaiser schützen!");
        assert_eq!(greeter.greet(), "Éljen Ferenc József császár!");
        assert_eq!(greeter.greet(), "Es lebe der Kaiser!");
    }
}
