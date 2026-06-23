trait C1Behavior {
    fn m1(&self) -> i32 {
        self.m2()
    }

    fn m2(&self) -> i32 {
        13
    }
}

trait C2Behavior: C1Behavior {
    fn c2_m1(&self) -> i32 {
        22
    }

    fn c2_m2(&self) -> i32 {
        23
    }

    fn m3(&self) -> i32 {
        // This simulates: super m1()
        // It calls C1's m1, but C1's m1 calls self.m2().
        C1Behavior::m1(self)
    }
}

struct C3;

impl C1Behavior for C3 {
    fn m1(&self) -> i32 {
        32
    }

    fn m2(&self) -> i32 {
        33
    }
}

impl C2Behavior for C3 {}

fn main() {
    let o3 = C3;

    let result = o3.m3();

    println!("{}", result);
}