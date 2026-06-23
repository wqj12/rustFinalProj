struct C1 {
    x: i32,
    y: i32,
}

impl C1 {
    fn new() -> C1 {
        C1 { x: 0, y: 0 }
    }

    fn setx1(&mut self, v: i32) {
        self.x = v;
    }

    fn sety1(&mut self, v: i32) {
        self.y = v;
    }

    fn getx1(&self) -> i32 {
        self.x
    }

    fn gety1(&self) -> i32 {
        self.y
    }
}

struct C2 {
    parent: C1,
    y: i32,
}

impl C2 {
    fn new() -> C2 {
        C2 {
            parent: C1::new(),
            y: 0,
        }
    }

    fn setx1(&mut self, v: i32) {
        self.parent.setx1(v);
    }

    fn sety1(&mut self, v: i32) {
        self.parent.sety1(v);
    }

    fn sety2(&mut self, v: i32) {
        self.y = v;
    }

    fn getx1(&self) -> i32 {
        self.parent.getx1()
    }

    fn gety1(&self) -> i32 {
        self.parent.gety1()
    }

    fn getx2(&self) -> i32 {
        self.parent.x
    }

    fn gety2(&self) -> i32 {
        self.y
    }
}

fn main() {
    let mut o2 = C2::new();

    o2.setx1(101);
    o2.sety1(102);
    o2.sety2(999);

    let result = vec![
        o2.getx1(),
        o2.gety1(),
        o2.getx2(),
        o2.gety2(),
    ];

    println!("{:?}", result);
}