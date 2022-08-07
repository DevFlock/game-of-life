use std::{thread, time};
use std::io;
use std;
use colored::Colorize;
use rand::seq::SliceRandom;


const RANDOM_GRID: bool = false;


#[derive(Clone, Debug, Copy, PartialEq)]
enum TileState {
    ALIVE,
    DEAD
}

#[derive(Clone, Debug, Copy)]
struct Tile {
    state: TileState
}

#[derive(Clone, Debug)]
struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>
}

impl Board {
    fn set(&mut self, x: usize, y: usize, state: TileState) {
        self.tiles[y][x].state = state;
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        if x >= self.height { return Tile{state: TileState::DEAD} }
        if y >= self.width { return Tile{state: TileState::DEAD} }
        self.tiles[x][y]
    }

    fn display(&self) {
        // Clear terminal
        print!("\x1B[2J\x1B[1;1H");

        for x in 0..self.height {
            for y in 0..self.width {
                match self.get(x, y).state {
                    TileState::ALIVE => print!("{}", "■ ".green().bold()),
                    TileState::DEAD => print!("  ")
                }
            }
            println!("");
        }
    }

    fn check_tile(&self, x: usize, y: usize) -> Tile {
        let mut neighbours = 0;
        let tile = self.get(x, y);

        // Check all tiles around the current tile
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x = (x as i32 + i) as usize;
                let y = (y as i32 + j) as usize;
                if self.get(x, y).state == TileState::ALIVE {
                    neighbours += 1;
                }
            }
        }

        if tile.state == TileState::ALIVE {
            return match neighbours {
                0 | 1 => Tile{state: TileState::DEAD},
                2 | 3 => Tile{state: TileState::ALIVE},
                _ => Tile{state: TileState::DEAD}
            }
        } else {
            return match neighbours {
                3 => Tile{state: TileState::ALIVE},
                _ => Tile{state: TileState::DEAD}
            }
        }

    }

    fn tick(&self) -> Board {
        let mut new_board = vec![vec![Tile{state: TileState::DEAD}; self.width]; self.height];

        for x in 0..self.height {
            for y in 0..self.width {
                new_board[x][y] = self.check_tile(x, y);
            }
        }

        Board {
            tiles: new_board,
            width: self.width,
            height: self.height
        }
    }
}

fn main() {
    println!("How large of a grid do you want?");
    println!("Width:");
    let mut width = String::new();
    io::stdin().read_line(&mut width)
        .ok()
        .expect("Failed to read line.");
    let width: usize = width.trim().parse().expect("Please type a number!");

    println!("\nHeight:");
    let mut height = String::new();
    io::stdin().read_line(&mut height)
        .ok()
        .expect("Failed to read line.");
    let height: usize = height.trim().parse().expect("Please type a number!");

    let mut grid = Board {
        tiles: vec![vec![Tile{state: TileState::DEAD}; width]; height],
        width,
        height
    };
   
    grid.set(1, 0, TileState::ALIVE);
    grid.set(2, 1, TileState::ALIVE);
    grid.set(0, 2, TileState::ALIVE);
    grid.set(1, 2, TileState::ALIVE);
    grid.set(2, 2, TileState::ALIVE);

    if RANDOM_GRID {
        for x in 0..grid.height {
            for y in 0..grid.width {
                grid.set(x, y, vec![TileState::ALIVE, TileState::DEAD].choose(&mut rand::thread_rng()).unwrap().clone());
            }
        }
    } else {
        let coordinates = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for (x, y) in coordinates {
            grid.set(x, y, TileState::ALIVE);
        }
    }
    
    loop {
        grid.display();
        grid = grid.tick();
        let duration = time::Duration::from_millis(200);
        thread::sleep(duration);
        println!("");
    }
}
