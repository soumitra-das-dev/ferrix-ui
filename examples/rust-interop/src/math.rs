//! Local Rust module demonstrating math functions, structs with methods, and enums.

pub const GOLDEN_RATIO: f64 = 1.618033988749895;

pub fn fib(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let next = a + b;
            a = b;
            b = next;
        }
        b
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Counter {
    pub value: i32,
    pub step: i32,
}

impl Counter {
    pub fn new(initial: i32, step: i32) -> Self {
        Self {
            value: initial,
            step,
        }
    }

    pub fn increment(&mut self) {
        self.value += self.step;
    }

    pub fn current(&self) -> i32 {
        self.value
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Idle,
    Running,
    Completed,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Idle => "Idle",
            Status::Running => "Running",
            Status::Completed => "Completed",
        }
    }
}
