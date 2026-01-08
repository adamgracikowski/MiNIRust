use std::collections::HashMap;
use std::hash::Hash;

#[macro_export]
macro_rules! string {
    ($s:expr) => {
        String::from($s)
    };
}

pub trait StateMachine<S> {
    fn step(&self, state: S) -> Option<S>;
}

#[macro_export]
macro_rules! impl_state_machine {
    ($name:ident, [ $( $from:tt -> $to:tt );* $(;)? ]) => {
        #[derive(Debug)]
        pub struct $name {
            map: std::collections::HashMap<i32, i32>,
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new() -> Self {
                let mut map = std::collections::HashMap::new();
                #[allow(non_upper_case_globals)]
                const END: i32 = -1;

                $(
                    map.insert($from, $to);
                )*

                Self { map }
            }
        }

        impl StateMachine<i32> for $name {
            fn step(&self, state: i32) -> Option<i32> {
                self.map.get(&state).copied()
            }
        }
    };
}

impl<S> StateMachine<S> for HashMap<S, S>
where
    S: Clone + Eq + Hash,
{
    fn step(&self, state: S) -> Option<S> {
        self.get(&state).cloned()
    }
}

pub fn join_machines<S, M1, M2>(x: M1, y: M2) -> Vec<Box<dyn StateMachine<S>>>
where
    S: 'static,
    M1: StateMachine<S> + 'static,
    M2: StateMachine<S> + 'static,
{
    vec![Box::new(x), Box::new(y)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_macro() {
        let s = string!("abc");
        assert_eq!(s, String::from("abc"));
    }

    #[test]
    fn test_state_machine_macro() {
        impl_state_machine!(MyMachine, [
            1 -> 3;
            2 -> 3;
            3 -> 4;
            4 -> END
        ]);

        let machine = MyMachine::new();

        assert_eq!(machine.step(1), Some(3));
        assert_eq!(machine.step(3), Some(4));
        assert_eq!(machine.step(4), Some(-1));
        assert_eq!(machine.step(-1), None);
    }

    #[test]
    fn test_hashmap_implementation() {
        let mut map = HashMap::new();
        map.insert("START", "NEXT");
        map.insert("NEXT", "STOP");

        assert_eq!(map.step("START"), Some("NEXT"));
        assert_eq!(map.step("NEXT"), Some("STOP"));
        assert_eq!(map.step("STOP"), None);
    }

    #[test]
    fn test_join_machines() {
        impl_state_machine!(MachineA, [ 1 -> 2 ]);
        impl_state_machine!(MachineB, [ 1 -> 3 ]);

        let a = MachineA::new();
        let b = MachineB::new();

        let joined = join_machines(a, b);

        assert_eq!(joined.len(), 2);
        assert_eq!(joined[0].step(1), Some(2));
        assert_eq!(joined[1].step(1), Some(3));
    }
}
