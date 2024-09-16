use std::fmt::Debug;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, Drawable, Mesh, PxScale, Rect, Text};
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

pub const COL_LEFT: Color = Color {
    r: 0.0,
    g: 0.8,
    b: 1.0,
    a: 1.0,
};

pub const COL_RIGHT: Color = Color {
    r: 1.0,
    g: 0.8,
    b: 0.0,
    a: 1.0,
};

fn lerp_color(a: &Color, b: &Color, s: f32) -> Color {
    Color {
        r: a.r * ( 1. - s ) + b.r * s,
        g: a.g * ( 1. - s ) + b.g * s,
        b: a.b * ( 1. - s ) + b.b * s,
        a: a.a * ( 1. - s ) + b.a * s,
    }
}
// --------------------- COLORS ---------------------

// ===================== MAIN =====================
fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("RustedPong", "Soma Deme")
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

fn draw_debug_center_marker(ctx: &mut Context, canvas: &mut Canvas, center: &Vec2) -> GameResult {
    let marker = Mesh::new_circle(ctx, DrawMode::fill(), Vec2 {x: 0.0, y: 0.0}, 3.0, 1.0, Color::RED)?;
    canvas.draw(&marker, *center);

    Ok(())
}

// ===================== PLAYER =====================
#[derive(Debug, Copy, Clone)]
enum Side {
    Left,
    Right,
}

enum Orientation {
    Vertical,
    Horizontal,
}

trait Entity {
    fn update(&mut self, ctx: &mut Context) -> () {
        self.lower_excitement();
    }

    fn trig_excited(&mut self) -> () {
        *self.get_excitement_ref() = 0.7;
    }

    fn lower_excitement(&mut self) -> () {
        let excitement = self.get_excitement_ref();
        if *excitement > 0. {
            *excitement -= 0.03;
        } else {
            *excitement = 0.;
        }
    }

    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_color(&self) -> Color;
    fn get_excitement(&self) -> f32;
    fn get_excitement_ref(&mut self) -> &mut f32;
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
        let excitement = self.get_excitement();
        let color_stroke = lerp_color(&self.get_color(), &Color::WHITE, excitement);
        let mut color_fill = self.get_color();
        color_fill.a *= 0.5 + excitement;

        let rectangle_fill = Mesh::new_rectangle(
            ctx, 
            DrawMode::fill(),
            Rect::new(
                -size.x / 2.0, 
                -size.y / 2.0, 
                size.x, 
                size.y
            ),
            color_fill,
        )?;
        let rectangle = Mesh::new_rectangle(
            ctx, 
            DrawMode::stroke(2.),
            Rect::new(
                -size.x / 2.0, 
                -size.y / 2.0, 
                size.x, 
                size.y
            ),
            color_stroke,
        )?;

        // Draw rectangle
        canvas.draw(&rectangle_fill, self.get_pos());
        canvas.draw(&rectangle, self.get_pos());

        // DEBUG
        // draw_debug_center_marker(ctx, canvas, &self.get_pos())?;

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
    curve_strength: f32,
    straight_strength: f32,
    color: Color,
    excitement: f32,
}

impl Player {
    fn new(ctx: &mut Context, side: &Side, controls: &Controls) -> Self {
        Player {
            side: *side,
            pos: Vec2 {
                // X position is based on side
                x: match side {
                    Side::Left => 100.,
                    Side::Right => ctx.gfx.drawable_size().0 - 100.,
                },
                y: ctx.gfx.drawable_size().1 / 2.
            },
            size: Vec2 {
                x: 20.0,
                y: 150.0
            },
            controls: *controls,
            speed: 7.0,
            curve_strength: 1.7,
            straight_strength: 1.05,
            color: match side {
                Side::Left  => COL_LEFT,
                Side::Right => COL_RIGHT,
            },
            excitement: 0.,
        }
    }

    fn check_collision(&self, other: &dyn Entity) -> bool {
        (self as &dyn Entity).check_collision(other)
    }

    fn hit(&mut self, ball: &mut Ball) -> GameResult {
        // Ball bounce (overwrite x position to avoid getting stuck)
        ball.bounce(&Orientation::Vertical)?;
        ball.pos.x = self.pos.x + match self.side {
            Side::Left  =>  (self.size.x + ball.size.x) / 2.,
            Side::Right => -(self.size.x + ball.size.x) / 2.,
        };
        
        // Add curve based on relative position
        let rel_diff = (ball.pos.y - self.pos.y) / (self.size.y / 2. + ball.size.y / 2.);
        ball.vel.y = ball.vel.x.abs() * rel_diff * self.curve_strength;
        
        // Add extra strength near center hit
        ball.vel.x *= 1. + (1. - rel_diff.abs()) * self.straight_strength;

        // Get excited
        (self as &mut dyn Entity).trig_excited();

        Ok(())
    }
}

impl Entity for Player {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
    }

    fn update(&mut self, ctx: &mut Context) -> () {

        if ctx.keyboard.is_key_pressed(self.controls.up) {
            self.pos.y -= self.speed;
        }
        if ctx.keyboard.is_key_pressed(self.controls.down) {
            self.pos.y += self.speed;
        }

        (self as &mut dyn Entity).lower_excitement();
    }
}
// --------------------- PLAYER ---------------------

// ===================== WALL =====================
struct Wall {
    pos: Vec2,
    size: Vec2,
    color: Color,
    excitement: f32,
}

impl Entity for Wall {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
    }
}
// --------------------- WALL ---------------------

// ===================== GOAL =====================
struct Goal {
    pos: Vec2,
    size: Vec2,
    side: Side,
    color: Color,
    excitement: f32,
}

impl Entity for Goal {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
    }
}
// --------------------- GOAL ---------------------

// ===================== BALL =====================
struct Ball {
    pos: Vec2,
    prev_pos: Vec2,
    vel: Vec2,
    size: Vec2,
    bounciness: f32,
    x_speed_limit: f32,
    color: Color,
    excitement: f32,
}

impl Ball {
    fn new(ctx: &mut Context) -> Self {
        Ball {
            pos: Vec2{ x: ctx.gfx.drawable_size().0 / 2., y: ctx.gfx.drawable_size().1 / 2. },
            prev_pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            size: Vec2{ x: 10.0, y: 10.0 },
            bounciness: 0.9,
            x_speed_limit: 8.,
            color: Color::WHITE,
            excitement: 0.,
        }
    }

    fn reset(&mut self, ctx: &mut Context) -> () {
        self.pos = Vec2{ x: ctx.gfx.drawable_size().0 / 2., y: ctx.gfx.drawable_size().1 / 2. };
        self.vel = Vec2::ZERO;
    }

    fn start(&mut self) -> () {
        self.vel = Vec2 { x: 3., y: 0. };
    }

    fn check_collision(&self, other: &dyn Entity) -> bool {
        (self as &dyn Entity).check_collision(other)
    }

    fn bounce(&mut self, surface_orientation: &Orientation) -> GameResult {
        // Bounce depending on surface orientation
        match surface_orientation {
            Orientation::Horizontal => {
                self.pos.y = self.prev_pos.y;
                self.vel.y *= -self.bounciness;
            },
            Orientation::Vertical   => {
                self.pos.x = self.prev_pos.x;
                self.vel.x *= -self.bounciness;
            }
        }

        // Limit X speed
        if self.vel.x > self.x_speed_limit {
            self.vel.x = self.x_speed_limit;
        } else if self.vel.x < -self.x_speed_limit {
            self.vel.x = -self.x_speed_limit;
        }

        // Get excited
        (self as &mut dyn Entity).trig_excited();
        
        Ok(())
    }
}

impl Entity for Ball {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
    }

    fn update(&mut self, ctx: &mut Context) -> () {
        // Update position based on speed
        self.prev_pos = self.pos;
        self.pos += self.vel;

        (self as &mut dyn Entity).lower_excitement();
    }
}
// --------------------- BALL ---------------------

// ===================== SCORE =====================
struct Score {
    left: u32,
    right: u32,
    text_pos: Vec2,
    color: Color,
}

impl Score {
    fn new(ctx: &mut Context) -> Self {
        Score {
            left: 0,
            right: 0,
            text_pos: Vec2 {
                x: ctx.gfx.drawable_size().0 / 2.,
                y: ctx.gfx.drawable_size().1 / 2.
            },
            color: lerp_color(&COL_BACKGROUND, &COL_FOREGROUND, 0.5),
        }
    }

    fn increment(&mut self, side: &Side) -> () {
        match side {
            Side::Left  => self.left  += 1,
            Side::Right => self.right += 1,
        }
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Assemble text
        let mut text: String = self.left.to_string();
        text += " ";
        text += self.right.to_string().as_str();

        // Create text item
        let mut score_text = Text::new(text);
        score_text.set_scale(PxScale::from(200.0));
        let bounding_box = score_text.dimensions(ctx).unwrap_or(Rect::one());
        let half_size = Vec2{ x: bounding_box.w / 2.0, y: bounding_box.h / 2.0 };

        // Draw text
        let mut param: DrawParam = DrawParam::new();
        param = param.color(self.color);
        param = param.offset(-(self.text_pos - half_size));
        canvas.draw(&score_text, param);

        // DEBUG
        //draw_debug_center_marker(ctx, canvas, &self.text_pos)?;

        Ok(())
    }
}
// --------------------- SCORE ---------------------

// ===================== TIMER =====================
enum TimerStatus {
    Ticking,
    Alarm,
    Inactive,
}

#[derive(Debug, Copy, Clone)]
enum TimerFunction {
    BallStart,
    ScoreRegister(Side),
}

struct Timer {
    status: TimerStatus,
    function: Option<TimerFunction>,
    time: f32,
}

impl Timer {
    fn new() -> Self {
        Timer {
            status: TimerStatus::Inactive,
            time: 0.,
            function: None,
        }
    }

    fn start(&mut self, function: TimerFunction) -> () {
        self.time = 5.;
        self.function = Some(function);
        self.status = TimerStatus::Ticking;
    }
    
    fn reset(&mut self) -> () {
        self.function = None;
        self.status = TimerStatus::Inactive;
    }

    fn get_function_to_execute(&mut self) -> Option<TimerFunction> {
        match self.status {
            TimerStatus::Alarm => {
                let func = self.function;
                self.reset();
                func
            },
            _ => None
        }
    }

    fn is_ticking(&self) -> bool {
        match self.status {
            TimerStatus::Ticking => true,
            _ => false
        }
    }

    fn update(&mut self) -> () {
        match self.status {
            TimerStatus::Ticking => {
                if self.time > 0. {
                    // Decrease time
                    self.time -= 0.1;
                } else {
                    // Set new state
                    self.status = TimerStatus::Alarm;
                }
            },
            TimerStatus::Alarm => (),
            TimerStatus::Inactive => (),
        }
    }
}
// --------------------- TIMER ---------------------

// ===================== GAME =====================
struct MyGame {
    players: Vec<Player>,
    walls: Vec<Wall>,
    goals: Vec<Goal>,
    ball: Ball,
    score: Score,
    timer: Timer,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let mut my_game = MyGame {
            players: vec![
                Player::new(ctx, &Side::Left, &Controls{ up:KeyCode::W, down:KeyCode::S }),
                Player::new(ctx, &Side::Right, &Controls{ up:KeyCode::Up, down:KeyCode::Down })
            ],
            walls: vec![
                Wall {
                    pos: Vec2 { x: ctx.gfx.drawable_size().0 / 2., y: 0. },
                    size: Vec2 { x: ctx.gfx.drawable_size().0, y: 80.0 },
                    color: COL_FOREGROUND,
                    excitement: 0.,
                },
                Wall {
                    pos: Vec2 { x: ctx.gfx.drawable_size().0 / 2., y: ctx.gfx.drawable_size().1 },
                    size: Vec2 { x: ctx.gfx.drawable_size().0, y: 80.0 },
                    color: COL_FOREGROUND,
                    excitement: 0.,
                },
                ],
                goals: vec![
                Goal {
                    pos: Vec2 { x: 0., y: ctx.gfx.drawable_size().1 / 2. },
                    size: Vec2 { x: 130.0, y: ctx.gfx.drawable_size().1 - 80. },
                    side: Side::Right,
                    color: lerp_color(&COL_LEFT, &COL_BACKGROUND, 0.5),
                    excitement: 0.,
                },
                Goal {
                    pos: Vec2 { x: ctx.gfx.drawable_size().0, y: ctx.gfx.drawable_size().1 / 2. },
                    size: Vec2 { x: 130.0, y: ctx.gfx.drawable_size().1 - 80. },
                    side: Side::Left,
                    color: lerp_color(&COL_RIGHT, &COL_BACKGROUND, 0.5),
                    excitement: 0.,
                },
            ],
            ball: Ball::new(ctx),
            score: Score::new(ctx),
            timer: Timer::new(),
        };

        // Start timer for first round
        my_game.timer.start(TimerFunction::BallStart);

        // Return
        my_game
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update timer and get events
        self.timer.update();
        match self.timer.get_function_to_execute() {
            Some(TimerFunction::BallStart) => self.ball.start(),
            Some(TimerFunction::ScoreRegister(side)) => {
                self.ball.reset(ctx);
                self.score.increment(&side);
                self.timer.start(TimerFunction::BallStart);
            },
            None => (),
        }

        // Collect all entities into a vector
        // TODO: This piece of code is a copy-paste available in draw() and update()
        // It would be nite to have this list when constructing MyGame
        let mut entity_refs: Vec<&mut dyn Entity> = Vec::new();
        for goal in &mut self.goals {
            entity_refs.push(goal as &mut dyn Entity);
        }
        for wall in &mut self.walls {
            entity_refs.push(wall as &mut dyn Entity);
        }
        for player in &mut self.players {
            entity_refs.push(player as &mut dyn Entity);
        }
        entity_refs.push(&mut self.ball as &mut dyn Entity);

        // Call update for each entity
        for entity_ref in entity_refs
        {
            entity_ref.update(ctx);
        }

        // Update player position (check for walls)
        for player in &mut self.players {
            for wall in &self.walls {
                if player.check_collision(wall) {
                    let wall_offset = ( wall.size.y + player.size.y ) / 2.;
                    player.pos.y = if player.pos.y < wall.pos.y {
                        wall.pos.y - wall_offset
                    } else {
                        wall.pos.y + wall_offset
                    };
                }
            }
        }
        
        // Check for hit
        for player in &mut self.players {
            if self.ball.check_collision(player) {
                player.hit(&mut self.ball)?;
            }
        }
        
        // Check for wall bounce
        for wall in &mut self.walls {
            if self.ball.check_collision(wall) {
                self.ball.bounce(&Orientation::Horizontal)?;
            }
        }
        
        // Check for score
        for goal in &mut self.goals {
            if self.ball.check_collision(goal) {
                (goal as &mut dyn Entity).trig_excited();
                if !self.timer.is_ticking() {
                    self.timer.start(TimerFunction::ScoreRegister(goal.side));
                }
            }
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create canvas to draw on
        let mut canvas = graphics::Canvas::from_frame(ctx, COL_BACKGROUND);

        // Collect all entities into a vector
        // TODO: This piece of code is a copy-paste available in draw() and update()
        // It would be nite to have this list when constructing MyGame
        let mut entity_refs: Vec<&mut dyn Entity> = Vec::new();
        for goal in &mut self.goals {
            entity_refs.push(goal as &mut dyn Entity);
        }
        for wall in &mut self.walls {
            entity_refs.push(wall as &mut dyn Entity);
        }
        for player in &mut self.players {
            entity_refs.push(player as &mut dyn Entity);
        }
        entity_refs.push(&mut self.ball as &mut dyn Entity);
        
        // Draw score
        self.score.draw(ctx, &mut canvas)?;

        // Call the draw function for each entity
        for entity_ref in entity_refs
        {
            entity_ref.draw(ctx, &mut canvas)?;
        }

        // End draw
        canvas.finish(ctx)
    }
}
// --------------------- GAME ---------------------
