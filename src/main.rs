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

// Define the Enemy struct to represent an enemy's position, speed, and whether it is dead or alive.
struct Enemy {
    x: f32,
    y: f32,
    speed: f32,
    dead: bool,
    // health: i32,
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
    dead: bool,
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

// Define the spawn_wave function to create a wave of enemies based on the specified number of rows, enemies per row, and spacing between enemies.
fn spawn_wave(enemies: &mut Vec<Enemy>, enemy_rows: usize, enemies_per_row: usize, enemy_spacing: f32) {
    for row in 0..enemy_rows {
        for column in 0..enemies_per_row {
            enemies.push(Enemy {
                x: 200.0 + (column as f32 * enemy_spacing),
                y: 100.0 + (row as f32 * enemy_spacing),
                speed: 100.0,
                dead: false,
            });
        }
    }
}

// The main function is the entry point of the game, where the game loop is executed and the game state is managed.
#[macroquad::main(my_window_config)]
async fn main() {
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

    // Initialize enemy attributes, including their positions, spacing, and the number of enemies per row and rows.
    let mut enemies: Vec<Enemy> = Vec::new();
    let enemy_spacing: f32 = 75.0;
    let enemies_per_row: usize = 8;
    let enemy_rows: usize = 4;

    // Populate the enemies vector with Enemy instances, calculating their positions based on the defined spacing and number of rows and columns.
    spawn_wave(&mut enemies, enemy_rows, enemies_per_row, enemy_spacing);

    // Initialize round number, maximum rounds, and player credits
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
                            dead: false,
                        });
                        lasers.push(Laser {
                            x: player.x,
                            y: player.y -20.0,
                            speed: 600.0,
                            angle: 12.0f32.to_radians(), 
                            dead: false,
                        });
                    } else {
                        lasers.push(Laser {
                            x: player.x,
                            y: player.y - 20.0,
                            speed: 600.0,
                            angle: 0.0,
                            dead: false,
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

                // Collision detection between lasers and enemies. If a laser hits an enemy, the enemy is removed, and the player earns credits.
                for laser in &mut lasers {
                    for enemy in &mut enemies {
                        // Check if the laser is within the bounds of the enemy's position (with a 15.0 unit margin)
                        // If so, increase the player's credits and remove the enemy from the game, and the laser.
                        if laser.x > enemy.x - 15.0 && laser.x < enemy.x + 15.0 &&
                            laser.y > enemy.y - 20.0 && laser.y < enemy.y + 20.0 {
                            credits += 150;
                            enemy.dead = true;
                            laser.dead = true;
                        }
                    }
                }

                // Remove enemies that have been hit by lasers here.
                enemies.retain(|enemy| !enemy.dead);

                // Remove lasers that have been hit by enemies here.
                lasers.retain(|laser| !laser.dead);

                if enemies.is_empty() {
                    round_number += 1;
                    current_state = GameState::RoundIntermission;
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
                    spawn_wave(&mut enemies, enemy_rows, enemies_per_row, enemy_spacing);
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
                draw_text("Press ESC to return to the round intermission screen.", 400.0, 800.0, 30.0, WHITE);

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

                if is_key_pressed(KeyCode::Escape) {
                    current_state = GameState::RoundIntermission;
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

        // Draw the enemies as a triangle
        for enemy in &enemies {
        let v1 = Vec2::new(enemy.x, enemy.y + 20.0);
        let v2 = Vec2::new(enemy.x - 15.0, enemy.y - 15.0);
        let v3 = Vec2::new(enemy.x + 15.0, enemy.y - 15.0);
        draw_triangle(v1, v2, v3, GREEN);
        }

        next_frame().await;
    }
}
