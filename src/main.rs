use macroquad::prelude::*;


enum GameState {
    Menu,
    InRound,
    RoundIntermission, // if in this state, able to open shop.
    Shop, // if in this state, able to buy upgrades.
    GameOver,
}

struct Player {
    x: f32,
    y: f32,
    speed: f32,
}

struct Laser {
    x: f32,
    y: f32,
    speed: f32,
}

enum Direction {
    Left,
    Right,
}

fn my_window_config() -> Conf {
    Conf {
        window_title: "Space Invaders".to_owned(),
        window_width: 1280,
        window_height: 960,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(my_window_config)]
async fn main() {

    let mut current_state = GameState::Menu;

    let mut player = Player {
        x: 640.0,
        y: 900.0,
        speed: 400.0,
    };

    let mut lasers: Vec<Laser> = Vec::new();

    loop {
        let delta_time = get_frame_time();
        clear_background(BLACK);

        match current_state {
            GameState::Menu => {
                draw_text("SPACE INVADERS", 400.0, 400.0, 60.0, WHITE);
                draw_text("Press ENTER to Start", 400.0, 500.0, 30.0, GRAY);
                if is_key_pressed(KeyCode::Enter) {
                    current_state = GameState::InRound;
                }
            }

            GameState::InRound => {
                let mut movement: Option<Direction> = None;

                if is_key_down(KeyCode::A) {
                    movement = Some(Direction::Left);
                } else if is_key_down(KeyCode::D) {
                    movement = Some(Direction::Right);
                }

                if let Some(dir) = movement {
                    match dir {
                        Direction::Left => {
                            player.x -= player.speed * delta_time;
                        }
                        Direction::Right => {
                            player.x += player.speed * delta_time;
                        }
                    }
                }

                if is_key_pressed(KeyCode::Space) {
                    lasers.push(Laser {
                        x: player.x,
                        y: player.y - 20.0,
                        speed: 600.0,
                    });
                }

                for laser in lasers.iter_mut() {
                    laser.y -= laser.speed * delta_time;
                }

                lasers.retain(|laser| laser.y > 0.0);

                for laser in lasers.iter() {
                    draw_rectangle(laser.x - 2.0, laser.y, 4.0, 15.0, RED);
                }

                if is_key_pressed(KeyCode::P) {
                    current_state = GameState::RoundIntermission;
                }
            }

            GameState::RoundIntermission => {
                draw_text("ROUND INTERMISSION", 400.0, 400.0, 60.0, WHITE);
                draw_text("Press B for shop", 400.0, 500.0, 60.0, WHITE);
                draw_text("Press ENTER to return to next round", 400.0, 600.0, 30.0, GRAY);
                if is_key_pressed(KeyCode::Enter) {
                    current_state = GameState::InRound;
                }
                if is_key_pressed(KeyCode::B) {
                    current_state = GameState::Shop;
                }
            }

            GameState::Shop => {
                draw_text("SHOP", 400.0, 200.0, 60.0, WHITE);

                draw_text("1. Double Lasers (2 angled lasers in opposite directions)", 400.0, 250.0, 25.0, GRAY);
                draw_text("2. Rapid Fire (faster firing rate)", 400.0, 350.0, 25.0, GRAY);
                draw_text("3. Shield (temporary invincibility)", 400.0, 450.0, 25.0, GRAY);





                draw_text("Press ENTER to return to next round", 400.0, 800.0, 30.0, GRAY);
                if is_key_pressed(KeyCode::Enter) {
                    current_state = GameState::InRound;
                }
            }

            GameState::GameOver => {
                draw_text("GAME OVER", 400.0, 450.0, 60.0, RED);
            } 
        }

        let v1 = Vec2::new(player.x, player.y - 30.0);
                let v2 = Vec2::new(player.x - 25.0, player.y + 25.0);
                let v3 = Vec2::new(player.x + 25.0, player.y + 25.0);

                draw_triangle(v1, v2, v3, PURPLE);

        next_frame().await;
    }
}
