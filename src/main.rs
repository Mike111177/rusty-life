#[derive(PartialEq, Copy, Clone, Eq)]
enum GOLCell {
    Dead,
    Alive,
}

impl std::fmt::Debug for GOLCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GOLCell::Alive => "█",
            GOLCell::Dead => " ",
        })
    }
}

const SIZE: usize = 20;
const ISIZE: isize = SIZE as isize;
type GOLBuffer = [[GOLCell; SIZE]; SIZE];

fn check_neighbor(x: isize, y: isize, buff: &GOLBuffer) -> bool {
    if x < 0 || x >= ISIZE || y < 0 || y >= ISIZE {
        return false;
    } else {
        return buff[x as usize][y as usize] == GOLCell::Alive;
    }
}

fn print_buff(buff: &GOLBuffer) {
    for row in buff {
        for cell in row {
            print!("{:?}", cell);
        }
        println!();
    }
}

fn main() {
    use GOLCell::{Alive, Dead};
    let mut buff_a: GOLBuffer = [[Dead; SIZE]; SIZE];
    let mut buff_b: GOLBuffer = [[Dead; SIZE]; SIZE];
    let mut front = &mut buff_a;
    let mut back = &mut buff_b;

    //Blinker
    front[3][3] = Alive;
    front[3][4] = Alive;
    front[3][5] = Alive;

    //Toad
    front[7][3] = Alive;
    front[7][4] = Alive;
    front[7][5] = Alive;
    front[8][4] = Alive;
    front[8][5] = Alive;
    front[8][6] = Alive;

    loop {
        for x in 0..ISIZE {
            for y in 0..ISIZE {
                //Count neighbors
                let mut count: i32 = 0;
                for n_x in -1..2 {
                    for n_y in [-1, 1] {
                        if check_neighbor(x + n_x, y + n_y, &front) {
                            count += 1;
                        }
                    }
                }
                for n_x in [-1, 1] {
                    if check_neighbor(x + n_x, y, &front) {
                        count += 1;
                    }
                }

                //Apply rules
                let cell = &front[x as usize][y as usize];
                back[x as usize][y as usize] = if (*cell == Alive && (count == 2 || count == 3))
                    || (*cell == Dead && count == 3)
                {
                    Alive
                } else {
                    Dead
                };
            }
        }
        std::mem::swap(&mut front, &mut back);
        print_buff(&front);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
