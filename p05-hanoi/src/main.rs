mod hanoi;

use hanoi::Hanoi;

fn main() {
    let mut h = Hanoi::new(6);
    h.print();
    let disks = h.disks();
    solve_hanoi(&mut h, disks, 0, 2, 1);
}

fn solve_hanoi(h: &mut Hanoi, n: u32, from: usize, to: usize, aux: usize) {
    if n == 1 {
        h.move_disk(from, to);
        return;
    }

    solve_hanoi(h, n - 1, from, aux, to);
    h.move_disk(from, to);
    solve_hanoi(h, n - 1, aux, to, from);
}
