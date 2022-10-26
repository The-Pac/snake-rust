use std::thread;
use std::time::Duration;
use colored::Colorize;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;

const X_SIZE_MAP: usize = 50;
const Y_SIZE_MAP: usize = 10;
const POINT: u32 = 10;
const WALL_CHAR: char = '#';
const APPLE_CHAR: char = '@';
const SNAKE_CHAR: char = 'O';
const SNAKE_TAILS_CHAR: char = '¤';

fn main() {
    let mut map: Vec<Vec<char>> = vec![vec![WALL_CHAR; X_SIZE_MAP]; Y_SIZE_MAP];
    let mut is_game_done: bool = false;
    let device_state = DeviceState::new();
    let mut keys: Vec<Keycode>;
    let mut points: u32 = 0;

    let mut snake: Snake = Snake {
        x: X_SIZE_MAP / 2,
        y: Y_SIZE_MAP / 2,
        icon: SNAKE_CHAR,
        tails: vec![],
        direction: 'Z',
    };

    generate_map(&mut map);

    //place snake
    map[snake.y][snake.x] = snake.icon;
    //place first apple
    place_apple(&mut map);

    while !is_game_done {
        keys = device_state.get_keys();
        let last_snake_position_x: usize;
        let last_snake_position_y: usize;

        for key in keys.iter() {
            match key {
                Keycode::D => {
                    if snake.direction != 'Q' {
                        if map[snake.y][snake.x + 1] != WALL_CHAR && map[snake.y][snake.x + 1] != SNAKE_TAILS_CHAR {
                            snake.direction = 'D';
                        } else {
                            is_game_done = true;
                        }
                    }
                }
                Keycode::Q => {
                    if snake.direction != 'D' {
                        if map[snake.y][snake.x - 1] != WALL_CHAR && map[snake.y][snake.x - 1] != SNAKE_TAILS_CHAR {
                            snake.direction = 'Q';
                        } else {
                            is_game_done = true;
                        }
                    }
                }
                Keycode::S => {
                    if snake.direction != 'Z' {
                        if map[snake.y + 1][snake.x] != WALL_CHAR && map[snake.y + 1][snake.x] != SNAKE_TAILS_CHAR {
                            snake.direction = 'S';
                        } else {
                            is_game_done = true;
                        }
                    }
                }
                Keycode::Z => {
                    if snake.direction != 'S' {
                        if map[snake.y - 1][snake.x] != WALL_CHAR && map[snake.y - 1][snake.x] != SNAKE_TAILS_CHAR {
                            snake.direction = 'Z';
                        } else {
                            is_game_done = true;
                        }
                    }
                }
                _ => {}
            }
        }

        map[snake.y][snake.x] = ' ';

        last_snake_position_x = snake.x;
        last_snake_position_y = snake.y;

        match snake.direction {
            'D' => {
                snake.x += 1;
            }
            'Q' => {
                snake.x -= 1;
            }
            'S' => {
                snake.y += 1;
            }
            'Z' => {
                snake.y -= 1;
            }
            _ => {}
        }

        if map[snake.y][snake.x] == APPLE_CHAR {
            let _ = &snake.tails.push(Tails {
                x: snake.x,
                y: snake.y,
                icon: SNAKE_TAILS_CHAR,
            });
            place_apple(&mut map);
            points += POINT;
        }

        if snake.tails.len() > 0 {
            let mut previous_x = 0;
            let mut previous_y = 0;
            let mut old_x;
            let mut old_y;
            for (tail_index, tail) in snake.tails.iter_mut().enumerate() {
                map[tail.y][tail.x] = ' ';
                match tail_index {
                    0 => {
                        previous_x = tail.x;
                        previous_y = tail.y;
                        tail.x = last_snake_position_x;
                        tail.y = last_snake_position_y;
                    }
                    _ => {
                        old_x = tail.x;
                        old_y = tail.y;
                        tail.x = previous_x;
                        tail.y = previous_y;
                        previous_x = old_x;
                        previous_y = old_y;
                    }
                }
                map[tail.y][tail.x] = tail.icon;
            }
        }
        map[snake.y][snake.x] = snake.icon;


        //Display map
        for column in &map {
            for cell in column {
                match *cell {
                    APPLE_CHAR => {
                        print!("{}", cell.to_string().red());
                    }
                    SNAKE_CHAR => {
                        print!("{}", cell.to_string().bright_yellow());
                    }
                    SNAKE_TAILS_CHAR => {
                        print!("{}", cell.to_string().yellow());
                    }
                    WALL_CHAR => {
                        print!("{}", cell.to_string().green());
                    }
                    _ => {
                        print!("{}", cell);
                    }
                }
            }
            println!();
        }
        println!("points: {}", points.to_string().magenta());

        thread::sleep(Duration::from_millis(100));
        clearscreen::clear().expect("failed to clear screen");
    };
}

fn generate_map(map: &mut Vec<Vec<char>>) {
    for (column_index, column) in map.iter_mut().enumerate() {
        for (cell_index, cell) in column.iter_mut().enumerate() {
            if column_index != 0 && cell_index != 0 && column_index != Y_SIZE_MAP - 1 && cell_index != X_SIZE_MAP - 1 {
                *cell = ' ';
            }
        }
    }
}

fn place_apple(map: &mut Vec<Vec<char>>) {
    let random_x = rand::thread_rng().gen_range(1..X_SIZE_MAP - 1);
    let random_y = rand::thread_rng().gen_range(1..Y_SIZE_MAP - 1);
    map[random_y][random_x] = APPLE_CHAR;
}


struct Snake {
    x: usize,
    y: usize,
    icon: char,
    tails: Vec<Tails>,
    direction: char,
}

struct Tails {
    x: usize,
    y: usize,
    icon: char,
}