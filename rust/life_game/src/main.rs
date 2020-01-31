use rand::{self, Rng};
use std::io::{stdout, Write};
use std::{thread, time};

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

fn get_char() -> char {
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0, 5);
    let chars = vec!['@', ':', '^', '=', '$'];
    chars[index]
}

fn print_table(table: &Vec<Vec<char>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", table[y][x]);
        }
        println!("");
    }
}

fn count_cells_around(table: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
    if y <= 0 || HEIGHT - 1 <= y || x <= 0 || WIDTH - 1 <= x {
        return -1;
    }
    let mut cnt = 0;
    for ny in y - 1..y + 2 {
        for nx in x - 1..x + 2 {
            if ny == x && nx == y {
                continue;
            }
            if table[ny][nx] == '0' {
                cnt += 1;
            }
        }
    }
    return cnt;
}

fn paint_cell(table: &mut Vec<Vec<char>>, y: usize, x: usize) {
    table[y][x] = get_char();
}

fn paint_table(table: &mut Vec<Vec<char>>) {
    for _ in 0..100 {
        let mut rng = rand::thread_rng();
        let y: usize = rng.gen_range(0, HEIGHT);
        let x: usize = rng.gen_range(0, WIDTH);
        paint_cell(table, y, x);
    }
}

fn start_game(table: &mut Vec<Vec<char>>, _height: usize, _width: usize) {
    stdout().flush().unwrap();
    loop {
        let interval = time::Duration::from_millis(1000);
        thread::sleep(interval);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let num_of_cells = count_cells_around(&table, y, x);
                if table[y][x] != '0' {
                    match &num_of_cells {
                        2 => (),
                        _ => table[y][x] = '0',
                    }
                } else {
                    if num_of_cells == 3 {
                        table[y][x] = get_char();
                    }
                }
            }
        }
        print_table(&table);
        println!("");
        stdout().flush().unwrap();
    }
}

fn main() {
    let mut table = vec![vec!['0'; WIDTH]; HEIGHT];
    paint_table(&mut table);
    start_game(&mut table, HEIGHT, WIDTH);
}
