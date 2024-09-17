use std::fmt::Debug;

use::macroquad::prelude::*;
use miniquad::window::screen_size;

// ===================== COLORS =====================
pub const COL_BACKGROUND: Color = Color {
    r: 0.03,
    g: 0.03,
    b: 0.03,
    a: 1.0,
};

pub const COL_FOREGROUND: Color = Color {
    r: 0.3,
    g: 0.3,
    b: 0.3,
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
#[macroquad::main("RustedPong")]
async fn main() {
    let mut rusted_pong = MyGame::new();

    loop {
        rusted_pong.update();
        rusted_pong.draw();

        next_frame().await
    }
}
// --------------------- MAIN ---------------------

// ===================== PLAYER =====================
#[derive(Debug, Copy, Clone, PartialEq)]
enum Side {
    Left,
    Right,
}

enum Orientation {
    Vertical,
    Horizontal,
}

trait ExcitedThing {
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

    fn calc_stroke_color(&self) -> Color {
        lerp_color(&self.get_base_color(), &WHITE, self.get_excitement())
    }
    
    fn calc_fill_color(&self) -> Color {
        let mut color_fill = self.get_base_color();
        color_fill.a *= 0.5 + self.get_excitement();
        color_fill
    }
    
    fn get_base_color(&self) -> Color;
    fn get_excitement(&self) -> f32;
    fn get_excitement_ref(&mut self) -> &mut f32;
}

trait Entity {
    /// Generic draw function for rectangle shaped entities
    fn draw(&self) -> () {
        // Create rectangle
        let pos = self.get_pos();
        let size = self.get_size();

        draw_rectangle(pos.x - size.x / 2., pos.y - size.y / 2., size.x, size.y, self.get_fill_color());
        draw_rectangle_lines(pos.x - size.x / 2., pos.y - size.y / 2., size.x, size.y, 4., self.get_stroke_color());
    }

    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_stroke_color(&self) -> Color;
    fn get_fill_color(&self) -> Color;
    fn resize(&mut self) -> ();
    fn update(&mut self) -> ();
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
    fn new(side: Side, controls: &Controls) -> Self {
        Player {
            side: side,
            pos: Player::calc_pos(side),
            size: Player::calc_size(),
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

    fn hit(&mut self, ball: &mut Ball) -> () {
        // Ball bounce (overwrite x position to avoid getting stuck)
        ball.bounce(&Orientation::Vertical);
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
        self.trig_excited();
    }

    fn calc_pos(side: Side) -> Vec2 {
        Vec2 {
            // X position is based on side
            x: match side {
                Side::Left => 100.,
                Side::Right => screen_width() - 100.,
            },
            y: screen_height() / 2.
        }
    }

    fn calc_size() -> Vec2 {
        Vec2 {
            x: 20.0,
            y: 150.0
        }
    }
}

impl ExcitedThing for Player {
    fn get_base_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
    }
}

impl Entity for Player {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_stroke_color(&self) -> Color {
        self.calc_stroke_color()
    }

    fn get_fill_color(&self) -> Color {
        self.calc_fill_color()
    }

    fn resize(&mut self) -> () {
        self.size = Player::calc_size();
        self.pos  = Player::calc_pos(self.side);
    }

    fn update(&mut self) -> () {
        // Flags that keep track of the intended movement of the player
        // Move up and move down CAN be true at the same time, in that case the player remains still
        let mut move_up = false;
        let mut move_down = false;

        // Handle touch screen input
        for touch in touches_local() {
            // Check if the touch is on the current player's side
            if (self.side == Side::Left && touch.position.x < 0.) || 
                (self.side == Side::Right && touch.position.x > 0.) {
                if touch.position.y < 0. {
                    move_up = true;
                } else {
                    move_down = true;
                }
            } 
        }

        // Handle keyboard input
        if is_key_down(self.controls.up) {
            move_up = true;
        }
        if is_key_down(self.controls.down) {
            move_down = true;
        }
        
        // Update position
        if move_up {
            self.pos.y -= self.speed;
        }
        if move_down {
            self.pos.y += self.speed;
        }

        // Lower excitement (aka entity glow)
        self.lower_excitement();
    }
}
// --------------------- PLAYER ---------------------

// ===================== WALL =====================
#[derive(Debug, Copy, Clone)]
enum WallSide {
    Top,
    Bottom,
}

struct Wall {
    pos: Vec2,
    size: Vec2,
    color: Color,
    excitement: f32,
    side: WallSide,
}

impl Wall {
    fn new(side: WallSide) -> Self {
        Wall {
            pos: Wall::calc_pos(side),
            size: Wall::calc_size(),
            color: COL_FOREGROUND,
            excitement: 0.,
            side: side,
        }
    }

    fn calc_pos(side: WallSide) -> Vec2 {
        Vec2 {
            x: screen_width() / 2.,
            y: match side {
                WallSide::Top    => 0.,
                WallSide::Bottom => screen_height(),
            }
        }
    }

    fn calc_size() -> Vec2 {
        Vec2 {
            x: screen_width() * 1.5,
            y: 80.0
        }
    }
}

impl ExcitedThing for Wall {
    fn get_base_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
    }
}

impl Entity for Wall {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_stroke_color(&self) -> Color {
        self.calc_stroke_color()
    }

    fn get_fill_color(&self) -> Color {
        self.calc_fill_color()
    }

    fn resize(&mut self) -> () {
        self.size = Wall::calc_size();
        self.pos  = Wall::calc_pos(self.side);
    }

    fn update(&mut self) -> () {
        self.lower_excitement();
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

impl Goal {
    fn new(side: Side) -> Self {
        Goal {
            pos: Goal::calc_pos(side),
            size: Goal::calc_size(),
            side: side,
            color: lerp_color(
                &match side {
                    Side::Left => COL_LEFT, 
                    Side::Right => COL_RIGHT,
                }, 
                &COL_BACKGROUND, 
                0.5
            ),
            excitement: 0.,
        }
    }

    fn calc_pos(side: Side) -> Vec2 {
        Vec2 {
            x: match side {
                Side::Left  => 0.,
                Side::Right => screen_width(),
            },
            y: screen_height() / 2.
        }
    }

    fn calc_size() -> Vec2 {
        Vec2 {
            x: 130.0,
            y: screen_height() - 80.
        }
    }
}

impl ExcitedThing for Goal {
    fn get_base_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
    }
}

impl Entity for Goal {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_stroke_color(&self) -> Color {
        self.calc_stroke_color()
    }

    fn get_fill_color(&self) -> Color {
        self.calc_fill_color()
    }

    fn resize(&mut self) -> () {
        self.size = Goal::calc_size();
        self.pos  = Goal::calc_pos(self.side);
    }

    fn update(&mut self) -> () {
        self.lower_excitement();
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
    fn new() -> Self {
        Ball {
            pos: Vec2{ x: screen_width() / 2., y: screen_height() / 2. },
            prev_pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            size: Vec2{ x: 10.0, y: 10.0 },
            bounciness: 0.9,
            x_speed_limit: 8.,
            color: WHITE,
            excitement: 0.,
        }
    }

    fn reset(&mut self) -> () {
        self.pos = Vec2{ x: screen_width() / 2., y: screen_height() / 2. };
        self.vel = Vec2::ZERO;
    }

    fn start(&mut self, side: Side) -> () {
        self.vel = Vec2 {
            x: match side {
                Side::Left => -3.,
                Side::Right => 3.,
            },
            y: 0.
        };
    }

    fn check_collision(&self, other: &dyn Entity) -> bool {
        (self as &dyn Entity).check_collision(other)
    }

    fn bounce(&mut self, surface_orientation: &Orientation) -> () {
        // Bounce depending on surface orientation
        match surface_orientation {
            Orientation::Horizontal => {
                self.pos.y = self.prev_pos.y;
                self.vel.y *= -self.bounciness;
            },
            Orientation::Vertical   => {
                self.pos.x = self.prev_pos.x;
                self.vel.x *= -self.bounciness;

                // Limit X speed because players could hit ball too fast
                if self.vel.x > self.x_speed_limit {
                    self.vel.x = self.x_speed_limit;
                } else if self.vel.x < -self.x_speed_limit {
                    self.vel.x = -self.x_speed_limit;
                }
            }
        }

        // Get excited
        self.trig_excited();
    }
}

impl ExcitedThing for Ball {
    fn get_base_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
    }
}

impl Entity for Ball {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_stroke_color(&self) -> Color {
        self.calc_stroke_color()
    }

    fn get_fill_color(&self) -> Color {
        self.calc_fill_color()
    }

    fn resize(&mut self) -> () {
        self.pos  = Vec2 {
            x: screen_width() / 2.,
            y: screen_height() / 2.,
        };
        self.vel = Vec2::ZERO;
    }

    fn update(&mut self) -> () {
        // Update position based on speed
        self.prev_pos = self.pos;
        self.pos += self.vel;

        self.lower_excitement();
    }
}
// --------------------- BALL ---------------------

// ===================== SCORE =====================
struct Score {
    left: u32,
    right: u32,
    color: Color,
    excitement_color: Color,
    excitement: f32,
}

impl Score {
    fn new() -> Self {
        Score {
            left: 0,
            right: 0,
            color: lerp_color(&COL_BACKGROUND, &COL_FOREGROUND, 0.5),
            excitement_color: WHITE,
            excitement: 0.,
        }
    }

    fn increment(&mut self, side: Side) -> () {
        match side {
            Side::Left  => {
                self.left += 1;
                self.excitement_color = COL_LEFT;
                self.trig_excited();
            },
            Side::Right  => {
                self.right += 1;
                self.excitement_color = COL_RIGHT;
                self.trig_excited();
            },
        }
    }

    fn draw(&self) -> () {
        // Assemble text
        let mut text: String = self.left.to_string();
        text += " ";
        text += self.right.to_string().as_str();
        let font_size = 250;

        // Get color
        let color = lerp_color(&self.color, &self.excitement_color, self.excitement);

        // Draw
        let text_center = get_text_center(&text, None, font_size, 1., 0.);
        draw_text(
            &text,
            screen_width() / 2. - text_center.x,
            screen_height() / 2. - text_center.y,
            font_size as f32,
            color
        );
    }

    fn update(&mut self) -> () {
        // TODO: This uses the ExcitedThing implementation with a different magic number
        // Consider reworking that function with some kind of parameter
        if self.excitement > 0. {
            self.excitement -= 0.01;
        } else {
            self.excitement = 0.;
        }
    }
}

impl ExcitedThing for Score {
    fn get_base_color(&self) -> Color {
        self.color
    }

    fn get_excitement(&self) -> f32 {
        self.excitement
    }

    fn get_excitement_ref(&mut self) -> &mut f32 {
        &mut self.excitement
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
    BallStart(Side),
    BallReset(Side),
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
    last_screen_size: Vec2,
}

impl MyGame {
    pub fn new() -> MyGame {
        let mut my_game = MyGame {
            players: vec![
                Player::new(Side::Left, &Controls{ up:KeyCode::W, down:KeyCode::S }),
                Player::new(Side::Right, &Controls{ up:KeyCode::Up, down:KeyCode::Down })
            ],
            walls: vec![
                Wall::new(WallSide::Top),
                Wall::new(WallSide::Bottom),
            ],
            goals: vec![
                Goal::new(Side::Left),
                Goal::new(Side::Right),
            ],
            ball: Ball::new(),
            score: Score::new(),
            timer: Timer::new(),
            last_screen_size: Vec2::from(screen_size()),
        };

        // Start timer for first round
        my_game.timer.start(TimerFunction::BallStart(Side::Left));

        // Return
        my_game
    }

    fn get_entity_refs<'a>(&'a mut self, entity_refs: &mut Vec<&'a mut dyn Entity>) -> () {
        // Init vector to make sure it's empty
        *entity_refs = Vec::new();

        // Get entity refs
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
    }

    /// Resizes and repositions every entity in case of the window being resized.
    /// Resets the current round but the score remains the same.
    fn resize(&mut self) -> () {
        // Collect all entities into a vector
        let mut entity_refs: Vec<&mut dyn Entity> = Vec::new();
        self.get_entity_refs(&mut entity_refs);

        // Call resize for each entity
        for entity_ref in entity_refs {
            entity_ref.resize();
        }

        // Ball was reset during resize, needs to be started again
        self.timer.start(TimerFunction::BallStart(Side::Left));
    }
}

trait EventHandler {
    fn update(&mut self) -> ();
    fn draw(&mut self) -> ();
}

impl EventHandler for MyGame {
    fn update(&mut self) -> () {
        // Check if window has been resized since las iteration
        let curr_screen_size = Vec2::from(screen_size());
        if curr_screen_size != self.last_screen_size {
            self.resize();
            self.last_screen_size = curr_screen_size;
        }

        // Update timer and get events
        self.timer.update();
        match self.timer.get_function_to_execute() {
            Some(TimerFunction::BallStart(side)) => {
                // Start ball
                self.ball.start(side)
            },
            Some(TimerFunction::BallReset(side)) => {
                // Start ball with some delay
                self.ball.reset();
                self.timer.start(TimerFunction::BallStart(side));
            },
            None => (),
        }

        // Collect all entities into a vector
        let mut entity_refs: Vec<&mut dyn Entity> = Vec::new();
        self.get_entity_refs(&mut entity_refs);

        // Call update for each entity
        for entity_ref in entity_refs {
            entity_ref.update();
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
                player.hit(&mut self.ball);
            }
        }
        
        // Check for wall bounce
        for wall in &mut self.walls {
            if self.ball.check_collision(wall) {
                self.ball.bounce(&Orientation::Horizontal);
            }
        }
        
        // Check for score
        self.score.update();
        for goal in &mut self.goals {
            if self.ball.check_collision(goal) {
                goal.trig_excited();
                // Check if the timer is ticking already, if so, the there is nothing to be done
                if !self.timer.is_ticking() {
                    // Register score for the opponent and start timer for ball reset
                    self.score.increment(match goal.side {
                        Side::Left => Side::Right,
                        Side::Right => Side::Left,
                    });
                    self.timer.start(TimerFunction::BallReset(goal.side));
                }
            }
        }
    }

    fn draw(&mut self) -> () {
        // Create canvas to draw on
        clear_background(COL_BACKGROUND);

        // Draw score
        self.score.draw();

        // Collect all entities into a vector
        let mut entity_refs: Vec<&mut dyn Entity> = Vec::new();
        self.get_entity_refs(&mut entity_refs);
        
        // Call the draw function for each entity
        for entity_ref in entity_refs {
            entity_ref.draw();
        }
    }
}
// --------------------- GAME ---------------------
