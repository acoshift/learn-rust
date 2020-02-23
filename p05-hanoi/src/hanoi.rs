pub struct Hanoi {
    rods: [Vec<u32>; 3],
    disks: u32,
}

impl Hanoi {
    pub fn new(n: u32) -> Hanoi {
        Hanoi {
            rods: [(0..n).rev().collect(), vec![], vec![]],
            disks: n,
        }
    }

    pub fn disks(&self) -> u32 {
        self.disks
    }

    pub fn move_disk(&mut self, from: usize, to: usize) {
        println!("move {} => {}", from, to);

        let d = self.rods[from].pop().expect("can not move");

        assert!(match self.rods[to].last() {
            Some(&t) => d < t,
            None => true,
        });
        self.rods[to].push(d);

        self.print();
    }

    pub fn print(&self) {
        for (i, rot) in self.rods.iter().enumerate() {
            print!("{}:: ", i);
            for d in rot {
                print!("{} ", d);
            }
            println!();
        }
        println!();
    }
}
