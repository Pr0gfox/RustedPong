use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, Rect};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::input::keyboard::*;

// ===================== COLORS =====================
pub const COL_BACKGROUND: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};

pub const COL_FOREGROUND: Color = Color {
    r: 0.8,
    g: 0.8,
    b: 0.8,
    a: 1.0,
};

pub const COL_BALL: Color = Color {
    r: 0.8,
    g: 0.8,
    b: 1.0,
    a: 1.0,
};
// --------------------- COLORS ---------------------

// ===================== MAIN =====================
fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
// --------------------- MAIN ---------------------

// ===================== PLAYER =====================
#[derive(Debug, Copy, Clone)]
enum Side {
    Left,
    Right,
}

trait Entity {
    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
}

impl dyn Entity {
    fn check_collision(&self, other: &dyn Entity) -> bool {
        // Get the top left and bottom right corners of the rectangles
        let a_top_l = self.get_pos()  - self.get_size()  / 2.0;
        let a_bot_r = self.get_pos()  + self.get_size()  / 2.0;
        let b_top_l = other.get_pos() - other.get_size() / 2.0;
        let b_bot_r = other.get_pos() + other.get_size() / 2.0;

        // Easy to calculate tha case of NOT colliding in case of rectangles
        if a_top_l.x > b_bot_r.x || a_top_l.y > b_bot_r.y ||
            b_top_l.x > a_bot_r.x || b_top_l.y > a_bot_r.y {
            false
        }
        else {
            true
        }
    }

    /// Generic draw function for rectangle shaped entities
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Create rectangle
        let size = self.get_size();
        let rectangle = Mesh::new_rectangle(
            ctx, 
            DrawMode::fill(),
            Rect::new(
                -size.x / 2.0, 
                -size.y / 2.0, 
                size.x, 
                size.y
            ),
            COL_FOREGROUND
        )?;

        // Draw rectangle
        canvas.draw(&rectangle, self.get_pos());

        // DEBUG
        let center = Mesh::new_circle(ctx, DrawMode::fill(), Vec2 {x: 0.0, y: 0.0}, 5.0, 1.0, Color::RED)?;
        canvas.draw(&center, self.get_pos());
        // DEBUG END

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
struct Controls {
    up: KeyCode,
    down: KeyCode,
}

struct Player {
    side: Side,
    pos: Vec2,
    size: Vec2,
    controls: Controls,
    speed: f32,
}

impl Player {
    fn new(side: &Side, controls: &Controls) -> Self {
        Player {
            side: *side,
            pos: Vec2 {
                // X position is based on side
                x: match side {
                    Side::Left => 40.0,
                    Side::Right => 300.0,
                },
                y: 150.0
            },
            size: Vec2 {
                x: 20.0,
                y: 200.0
            },
            controls: *controls,
            speed: 7.0,
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {

        if ctx.keyboard.is_key_pressed(self.controls.up) {
            self.pos.y -= self.speed;
        }
        if ctx.keyboard.is_key_pressed(self.controls.down) {
            self.pos.y += self.speed;
        }

        Ok(())
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        (self as &dyn Entity).draw(ctx, canvas)
    }
}

impl Entity for Player {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }
}
// --------------------- PLAYER ---------------------

// ===================== BALL =====================
struct Ball {
    pos: Vec2,
    vel: Vec2,
    size: f32,
}

impl Ball {
    fn new() -> Self {
        Ball {
            pos: Vec2{ x: 300.0, y: 300.0 },
            vel: Vec2{ x: 1.0, y: 0.0 },
            size: 7.0,
        }
    }
    
    fn update(&mut self) -> GameResult {
        self.pos += self.vel;
        
        Ok(())
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Create circle
        let circle = Mesh::new_circle(
            ctx, 
            DrawMode::fill(), 
            Vec2 {x: 0.0, y: 0.0}, 
            self.size, 
            1.0, 
            COL_BALL
        )?;

        // Draw circle
        canvas.draw(&circle, self.pos);

        Ok(())
    }
}
// --------------------- BALL ---------------------

// ===================== GAME =====================
struct MyGame {
    players: Vec<Player>,
    ball: Ball,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            players: vec![
                Player::new(&Side::Left, &Controls{ up:KeyCode::W, down:KeyCode::S }),
                Player::new(&Side::Right, &Controls{ up:KeyCode::Up, down:KeyCode::Down })
                ],
            ball: Ball::new(),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update player position
        for player in &mut self.players {
            player.update(ctx)?;
        }

        // Update ball position
        self.ball.update()?;

        // Update score
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create canvas to draw on
        let mut canvas = graphics::Canvas::from_frame(ctx, COL_BACKGROUND);

        // Draw players
        for player in &mut self.players {
            player.draw(ctx, &mut canvas)?;
        }

        // Draw ball
        self.ball.draw(ctx, &mut canvas)?;

        // End draw
        canvas.finish(ctx)
    }
}
// --------------------- GAME ---------------------
