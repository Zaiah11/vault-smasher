extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Key, Button, PressEvent};
use piston::window::WindowSettings;
use rand::Rng;


const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

// Settings ===============================================================================================
const SCREEN_WIDTH: u32 = 960;
const SCREEN_HEIGHT: u32 = 540;
const SCALE: i32 = 10;
// ========================================================================================================

fn normalize_pixel(size: i32) -> i32 {
    size * SCALE
}

enum Direction {
    None,
    Up,
    Down,
    Left,
    Right
}

struct Location ( i32, i32 );

struct Snake {
    location: Vec<Location>,
    direction: Direction
}

impl Snake {
    pub fn new() -> Snake {
        Snake { 
            location:  vec![Location (SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 /2)],
            direction: Direction::None
        } 
    }

    fn move_self(&mut self) {
        let Location (mut x, mut y) = self.location.pop().unwrap();

        match self.direction {
            Direction::Up => { y -= normalize_pixel(1)},
            Direction::Down => { y += normalize_pixel(1)},
            Direction::Left => { x -= normalize_pixel(1)},
            Direction::Right => {x += normalize_pixel(1)},
            Direction::None => {}
        }

        self.location.insert(0, Location (x, y));
    }
}

struct Food {
    location: Location
}

impl Food {
    pub fn new() -> Food {
        Food {
            location: Location (-10, -10)
        }
    }
}

pub struct App {
    current_tick: f64,
    tick_window: f64,

    snake: Snake,
    food: Food
}

impl App {
    pub fn new() -> App {
        App {
            current_tick: 0.0,
            tick_window: 0.05,

            snake: Snake::new(),
            food: Food::new()
        }
    }
    
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, normalize_pixel(1) as f64);

        gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            for Location (s_x, s_y) in self.snake.location.iter() {
                let s_transform = c
                    .transform
                    .trans(*s_x as f64, *s_y as f64);
                rectangle(RED, square, s_transform, gl);
            }

            let Location (f_x, f_y) = self.food.location;
            let f_transform = c
                .transform
                .trans(f_x as f64, f_y as f64);
            rectangle(RED, square, f_transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.current_tick += args.dt;
        if self.current_tick > self.tick_window {
            self.current_tick -= self.tick_window;
            self.snake.move_self();
        }
    }

    fn new_food_spawn_location(&self) -> Location {
        let mut rng = rand::thread_rng();

        let new_location = Location (
            rng.gen_range(0..(SCREEN_WIDTH / 10) as i32), 
            rng.gen_range(0..(SCREEN_HEIGHT / 10) as i32)
        );

        if self.location_overlaps_player(&new_location) {
            return self.new_food_spawn_location();
        }

        new_location
    }
    
    fn location_overlaps_player(&self, location: &Location) -> bool {
        let Location (f_x, f_y) = self.food.location;

        for Location (s_x, s_y) in &self.snake.location {
            if f_x == *s_x && f_y == *s_y {
                return true;
            }
        }

        false
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new();
    let Location (x, y) = app.new_food_spawn_location();
    println!("x: {}, y: {}", x, y);

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut gl);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => app.snake.direction = Direction::Up,
                Key::S => app.snake.direction = Direction::Down,
                Key::A => app.snake.direction = Direction::Left,
                Key::D => app.snake.direction = Direction::Right,
                Key::Space => app.snake.direction = Direction::None,
                _ => {}
            }
        };
    }
}