use macroquad::prelude::*;


// Define the GameState enum to represent the different states of the game,
// including the menu, in-round gameplay, round intermission, shop, and game over.
enum GameState {
    Menu,
    InRound,
    RoundIntermission, // if in this state, able to open shop.
    Shop, // if in this state, able to buy upgrades.
    GameOver,
}

// Define the Player struct to represent the player's position and speed in the game.
struct Player {
    x: f32,
    y: f32,
    speed: f32,
}

// Define the Shield struct to represent whether the player owns and has activated the shield upgrade.
struct Shield {
    owned: bool,
    active: bool,
}

// Define the DoubleLasers struct to represent whether the player owns the double lasers upgrade.
struct DoubleLasers {
    owned: bool,
}

// Define the Laser struct to represent a laser shot by the player, including its position, speed, and angle.
struct Laser {
    x: f32,
    y: f32,
    speed: f32, 
    angle: f32,
}

// Define the direction enum to represent player movement directions (left or right).
enum Direction {
    Left,
    Right,
}

// Define the window configuration for the game, including title, dimensions, and other settings.
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
s
    // Initialize game state and player attributes
    let mut current_state = GameState::Menu;
    let mut player = Player {
        x: 640.0,
        y: 900.0,
        speed: 400.0,
    };
    let mut shield = Shield {
        owned: false,
        active: false,
    };
    let mut double_lasers = DoubleLasers {
        owned: false,
    };
    let mut lasers: Vec<Laser> = Vec::new();
    let mut round_number: u32 = 1;
    let round_max: u32 = 255;

    let mut credits: i32 = 25000;

    // Main game loop
    loop {
        let delta_time = get_frame_time();
        clear_background(BLACK);

        match current_state {
            // Menu state displays the game title and instructions to start the game.
            GameState::Menu => {
                draw_text("SPACE INVADERS", 400.0, 400.0, 60.0, WHITE);
                draw_text("Press ENTER to Start", 400.0, 500.0, 30.0, GRAY);
                if is_key_pressed(KeyCode::Enter) {
                    current_state = GameState::InRound;
                }
            }

            // InRound state handles player movement, shooting, and round progression. 
            // The player can move left and right, shoot lasers, and progress to the next round or enter the shop.
            GameState::InRound => {
                let mut movement: Option<Direction> = None;

                // Handle player movement based on key presses. The player can move left with the A key and right with the D key.
                if is_key_down(KeyCode::A) {
                    movement = Some(Direction::Left);
                } else if is_key_down(KeyCode::D) {
                    movement = Some(Direction::Right);
                }

                // Update player position based on movement direction and speed, 
                // scaled by delta_time to ensure consistent movement across different frame rates.
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

                // Keep the player within the screen bounds
                if player.x < 25.0 {
                    player.x = 25.0;
                } else if player.x > 1255.0 {
                    player.x = 1255.0;
                }

                // Handle shooting lasers when the spacebar is pressed. 
                // If the player has the double lasers upgrade, shoot two angled lasers; otherwise, shoot a single straight laser.
                if is_key_pressed(KeyCode::Space) {
                    if double_lasers.owned {
                        lasers.push(Laser {
                            x: player.x,
                            y: player.y -20.0,
                            speed: 600.0,
                            angle: -12.0f32.to_radians(), 
                        });
                        lasers.push(Laser {
                            x: player.x,
                            y: player.y -20.0,
                            speed: 600.0,
                            angle: 12.0f32.to_radians(), 
                        });
                    } else {
                        lasers.push(Laser {
                            x: player.x,
                            y: player.y - 20.0,
                            speed: 600.0,
                            angle: 0.0,
                        });
                    }
                }

                // Allow the player to skip to the next round by pressing C. This is useful for testing and debugging.
                if is_key_pressed(KeyCode::C) {
                    round_number += 1;
                }

                // Update laser positions and remove lasers that have gone off-screen
                for laser in lasers.iter_mut() {
                    laser.x += laser.angle.sin() * laser.speed * delta_time;
                    laser.y -= laser.angle.cos() * laser.speed * delta_time;
                }

                // Remove lasers that have gone off-screen
                lasers.retain(|laser| laser.y > 0.0);

                // Draw lasers
                for laser in lasers.iter() {
                    draw_rectangle_ex(
                        laser.x - 2.0,
                        laser.y,
                        4.0,
                        15.0,
                        DrawRectangleParams {
                            rotation: laser.angle,
                            color: RED,
                            ..Default::default()
                        }
                    );
                }

                // Allow the player to skip to the next round by pressing P. This is useful for testing and debugging.
                if is_key_pressed(KeyCode::P) {
                    current_state = GameState::RoundIntermission;
                }

                // Display round number and credits
                draw_text(&format!("ROUND: {}", round_number), 20.0, 40.0, 30.0, WHITE);
                draw_text(&format!("CREDITS: {}", credits), 20.0, 80.0, 30.0, WHITE);

                if round_number >= round_max {
                    // Mimicking Black Ops III style of ending the game when the player reaches the max round.
                    std::process::abort();
                }
            }

            // RoundIntermission state allows the player to either go to the shop or return to the next round.
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

            // Shop state allows the player to buy upgrades if they have enough credits.
            GameState::Shop => {

                draw_text("SHOP", 400.0, 200.0, 60.0, WHITE);

                if double_lasers.owned {
                    draw_text("Double Lasers: OWNED", 400.0, 250.0, 25.0, GREEN);
                } else {
                    draw_text("($1250) 1. Double Lasers (2 angled lasers in opposite directions)", 400.0, 250.0, 25.0, GRAY);
                }
                
                if shield.owned {
                    draw_text("Shield: OWNED", 400.0, 300.0, 25.0, GREEN);
                } else {
                    draw_text("($2500) 2. Shield (Protects player from one hit)", 400.0, 300.0, 25.0, GRAY);
                }

                draw_text("Press corrosponding number to buy.", 400.0, 750.0, 30.0, GRAY);
                draw_text("Press ENTER to return to next round", 400.0, 800.0, 30.0, WHITE);

                if is_key_pressed(KeyCode::Key1) {
                    if credits >= 1250 && !double_lasers.owned {
                        credits -= 1250;
                        double_lasers.owned = true;
                        // Implement double lasers upgrade logic here
                    }
                }

                if is_key_pressed(KeyCode::Key2) {
                    if credits >= 2500 && !shield.owned {
                        credits -= 2500;
                        shield.owned = true;
                        // Implement shield upgrade logic here
                    }
                }


                if is_key_pressed(KeyCode::Enter) {
                    current_state = GameState::InRound;
                }
            }

            // GameOver state displays a game over message and allows the player to restart the game.
            GameState::GameOver => {
                draw_text("GAME OVER", 400.0, 450.0, 60.0, RED);
            } 
        }

        // Draw the player as a triangle
        let v1 = Vec2::new(player.x, player.y - 30.0);
        let v2 = Vec2::new(player.x - 25.0, player.y + 25.0);
        let v3 = Vec2::new(player.x + 25.0, player.y + 25.0);
        draw_triangle(v1, v2, v3, PURPLE);


        next_frame().await;
    }
}
