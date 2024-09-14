// //! The simplest possible example that does something.
// #![allow(clippy::unnecessary_wraps)]

// use ggez::{
//     event,
//     glam::*,
//     graphics::{self, Color},
//     Context, GameResult,
// };

// // COLORS

// struct MainState {
//     pos_x: f32,
//     circle: graphics::Mesh,
// }

// impl MainState {
//     fn new(ctx: &mut Context) -> GameResult<MainState> {
//         let circle = graphics::Mesh::new_circle(
//             ctx,
//             graphics::DrawMode::fill(),
//             vec2(0., 0.),
//             100.0,
//             2.0,
//             Color::WHITE,
//         )?;

//         Ok(MainState { pos_x: 0.0, circle })
//     }
// }

// impl event::EventHandler<ggez::GameError> for MainState {
//     fn update(&mut self, _ctx: &mut Context) -> GameResult {
//         self.pos_x = self.pos_x % 800.0 + 1.0;
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         let mut canvas =
//             graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

//         canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));

//         canvas.finish(ctx)?;

//         Ok(())
//     }
// }

// pub fn main() -> GameResult {
//     let cb = ggez::ContextBuilder::new("super_simple", "ggez");
//     let (mut ctx, event_loop) = cb.build()?;
//     let state = MainState::new(&mut ctx)?;
//     event::run(ctx, event_loop, state)
// }



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