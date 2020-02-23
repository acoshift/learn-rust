fn main() {
    let r1 = Rect { w: 10, h: 5 };
    let r2 = Rect { w: 5, h: 7 };
    let l = larger(&r1, &r2);
    println!("larger between {:?} and {:?} is\n{:#?}", r1, r2, l);
}

#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.h * self.w
    }
}

impl Area for Rect {
    fn area(&self) -> u32 {
        self.area()
    }
}

trait Area {
    fn area(&self) -> u32;
}

// impl PartialOrd for Rect {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.area().cmp(&other.area()))
//     }
// }
//
// impl PartialEq for Rect {
//     fn eq(&self, other: &Self) -> bool {
//         self.w == other.w && self.h == other.h
//     }
// }

fn larger<'a, T>(r1: &'a T, r2: &'a T) -> &'a T
where
    T: Area,
{
    if r1.area() > r2.area() {
        r1
    } else {
        r2
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;

    #[test]
    fn rect_area() {
        assert_eq!(Rect { w: 3, h: 4 }.area(), 12);
    }
}
