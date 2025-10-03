pub struct Collatz;

impl Collatz {
    pub fn verify(&self, number: u64, iter: u32) -> bool {
        let mut current = number;
        for _ in 0..iter {
            if self.meet_condition(current) {
                return true;
            }
            current = self.step(current);
        }
        self.meet_condition(current)
    }

    pub fn verify_many(&self, numbers: &[u64], iter: u32) -> Vec<bool> {
        numbers.iter().map(|n| self.verify(*n, iter)).collect()
    }

    fn step(&self, n: u64) -> u64 {
        if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
    }

    fn meet_condition(&self, n: u64) -> bool {
        n == 1
    }
}
