use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, DrawMode, Rect};
use ggez::event::{self, EventHandler};

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
// --------------------- COLORS ---------------------

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

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, COL_BACKGROUND);
        // Draw code here...

        // Draw rectangle
        let rectangle: Mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(-10.0, -100.0, 20.0, 200.0), COL_FOREGROUND)?;
        canvas.draw(&rectangle, ggez::glam::Vec2::new(150.0, 380.0));
        // Draw rectangle end


        // graphics::Drawable::draw(&self, canvas, param);
        // canvas.draw(drawable, param);

        canvas.finish(ctx)
    }
}