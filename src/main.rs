use std::thread;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;

const X_SIZE_MAP: usize = 50;
const Y_SIZE_MAP: usize = 10;
const WALL_CHAR: char = '#';
const APPLE_CHAR: char = '@';
const SNAKE_CHAR: char = '#';
const SNAKE_TAILS_CHAR: char = '^';

fn main() {
    let mut map: Vec<Vec<char>> = vec![vec![WALL_CHAR; X_SIZE_MAP]; Y_SIZE_MAP];
    let mut is_game_done: bool = false;
    let device_state = DeviceState::new();
    let mut keys: Vec<Keycode>;

    let mut snake: Snake = Snake {
        x: X_SIZE_MAP / 2,
        y: Y_SIZE_MAP / 2,
        icon: SNAKE_CHAR,
        tails: vec![],
    };

    generate_map(&mut map);

    //place snake
    map[snake.y][snake.x] = snake.icon;
    //place first apple
    place_apple(&mut map);

    while !is_game_done {
        keys = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::D => {
                    if map[snake.y][snake.x + 1] != WALL_CHAR {
                        if map[snake.y][snake.x + 1] == APPLE_CHAR {
                            snake.tails.push(Tails {
                                x: snake.x,
                                y: snake.y,
                                icon: SNAKE_TAILS_CHAR,
                            });
                            place_apple(&mut map);
                        }
                        map[snake.y][snake.x] = ' ';
                        snake.x = snake.x + 1;
                    } else {
                        is_game_done = true;
                    }
                }
                Keycode::Q => {
                    if map[snake.y][snake.x - 1] != WALL_CHAR {
                        if map[snake.y][snake.x - 1] == APPLE_CHAR {
                            snake.tails.push(Tails {
                                x: snake.x,
                                y: snake.y,
                                icon: SNAKE_TAILS_CHAR,
                            });
                            place_apple(&mut map);
                        }
                        map[snake.y][snake.x] = ' ';
                        snake.x = snake.x - 1;
                    } else {
                        is_game_done = true;
                    }
                }
                Keycode::S => {
                    if map[snake.y + 1][snake.x] != WALL_CHAR {
                        if map[snake.y + 1][snake.x] == APPLE_CHAR {
                            snake.tails.push(Tails {
                                x: snake.x,
                                y: snake.y,
                                icon: SNAKE_TAILS_CHAR,
                            });
                            place_apple(&mut map);
                        }
                        map[snake.y][snake.x] = ' ';
                        snake.y = snake.y + 1;
                    } else {
                        is_game_done = true;
                    }
                }
                Keycode::Z => {
                    if map[snake.y - 1][snake.x] != WALL_CHAR {
                        if map[snake.y - 1][snake.x] == APPLE_CHAR {
                            snake.tails.push(Tails {
                                x: snake.x,
                                y: snake.y,
                                icon: SNAKE_TAILS_CHAR,
                            });
                            place_apple(&mut map);
                        }
                        if snake.tails.len() > 0 {
                            let last_x = 0;
                            let last_y = 0;
                            for (tail_index, tail) in snake.tails.iter_mut().enumerate() {
                                match tail_index {
                                    0 => {
                                        tail.x = snake.x;
                                        tail.y = snake.y;
                                    }
                                    _ => {
                                        tail.x = snake.tails[tail_index - 1].x + 1;
                                        tail.y = snake.tails[tail_index - 1].y;
                                    }
                                }
                            }
                        }
                        map[snake.y][snake.x] = ' ';
                        snake.y = snake.y - 1;
                    } else {
                        is_game_done = true;
                    }
                }
                _ => {}
            }
            map[snake.y][snake.x] = snake.icon;
            if snake.tails.len() > 0 {
                for tail in &snake.tails {
                    map[tail.y][tail.x] = SNAKE_TAILS_CHAR;
                }
            }
        }

        //Display map
        for column in &map {
            for cell in column {
                print!("{}", cell);
            }
            println!();
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        thread::sleep(Duration::from_millis(200))
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
}

struct Tails {
    x: usize,
    y: usize,
    icon: char,
}