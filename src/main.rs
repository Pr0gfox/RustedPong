use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, Rect};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;

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

struct Player {
    side: Side,
    pos: Vec2,
    size: Vec2,
}

impl Player {
    fn new(side: &Side) -> Self {
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
        }
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {

        // Create rectangle
        let rectangle = Mesh::new_rectangle(
            ctx, 
            DrawMode::fill(),
            Rect::new(
                -self.size.x / 2.0, 
                -self.size.y / 2.0, 
                self.size.x, 
                self.size.y
            ),
            COL_FOREGROUND
        )?;

        // DEBUG
        let center = Mesh::new_circle(ctx, DrawMode::fill(), Vec2 {x: 0.0, y: 0.0}, 5.0, 1.0, Color::RED)?;
        canvas.draw(&center, self.pos);
        // DEBUG END

        // Draw rectangle
        canvas.draw(&rectangle, self.pos);

        Ok(())
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
            players: vec![Player::new(&Side::Left), Player::new(&Side::Right)],
            ball: Ball::new(),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update player position

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
