// TODO use f64
// add enzyme class
// add random spawn conditions
// add spawn cooldown
// add colision physics
// add substrate
// add ability for substrate to turn into product through enzyme
// competetive inhibitors
// non-competetive inhibitors
// add nature reaction and subatomic repel

extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::mouse::{self, MouseButton};
use sdl2::rect::Point;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const ENZYME_A_BMP_PATH: &str = "enzymeA.bmp";

fn main() {
    // initalize sdl and canvas
    let context: sdl2::Sdl = sdl2::init()
        .expect("Failed to create sdl context.");
    let video_sub_system: sdl2::VideoSubsystem = context.video()
        .expect("Failed to create video subsystem");
    let mut window: sdl2::video::Window = video_sub_system.window("Enzyme Simulation", WIDTH, HEIGHT).build()
        .expect("Failed to create window.");
    window.set_bordered(false);
    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build()
        .expect("Failed to build canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp("enzymeA.bmp").unwrap())
        .expect("Failed to create texture.");

    let entity = Entity::new(texture, 50, (WIDTH as i32 / 2, HEIGHT as i32 / 2), (1, 1));
    let mut entities = vec![entity];

    let add_enzyme_a_button = sdl2::rect::Rect::new(WIDTH as i32 - 125, 25, 100, 50);
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

    'game_loop: loop {
        canvas.clear();
        for e in &mut entities {
            e.update();
            e.display(&mut canvas);
        }
        canvas.fill_rect(add_enzyme_a_button).expect("Failed to draw rectangle.");
        canvas.present();

        let mut event_pump = context.event_pump().expect("Failed to poll events."); // poll for user input
        // check to see if the user wants to quit
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'game_loop;
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        let point = Point::new(x, y);
                        if add_enzyme_a_button.contains_point(point) {
                            let texture = texture_creator
                                .create_texture_from_surface(sdl2::surface::Surface::load_bmp("enzymeA.bmp").unwrap())
                                .expect("Failed to create texture.");
                            let new_entity = Entity::new(texture, 50, (WIDTH as i32 / 2, HEIGHT as i32 / 2), (1, 1));
                            entities.push(new_entity);
                        }
                    }
                }
                _ => {}
            }
        }
    }

}

struct Entity<'a> {
    sprite: sdl2::render::Texture<'a>,
    radius: usize,
    position: (i32, i32),
    velocity: (i32, i32),
    rect: sdl2::rect::Rect,
}

impl<'a> Entity<'a> {
    pub fn new(sprite: sdl2::render::Texture<'a>, radius: usize, position: (i32, i32), velocity: (i32, i32)) -> Entity{
        let rect = sdl2::rect::Rect::new(position.0 - radius as i32, position.1 + radius as i32,
            radius as u32 * 2, radius as u32* 2);
        Entity {sprite, radius, position, velocity, rect}
    }
    pub fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.bounds_check();
    }
    fn bounds_check(& mut self) {
        let radius = self.radius as i32;
        // check x bounds
        if (self.position.0 - radius <= 0 && self.velocity.0 < 0) || (self.position.0 + radius >= WIDTH as i32 && self.velocity.0 > 0) {
            self.velocity.0 *= -1;
        }
        // check y bounds
        if (self.position.1 - radius <= 0) || (self.position.1 + radius >= HEIGHT as i32) {
            self.velocity.1 *= -1;
        }
    }
    pub fn display(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.rect.set_x(self.position.0 - self.radius as i32);
        self.rect.set_y(self.position.1 - self.radius as i32);
        canvas.copy(&self.sprite, None, self.rect).expect("Failed to display entity.");
    }
}

/*
enum Enzyme_type<'a> {
    A {bmp_path: &'a str, ideal_temp: usize, idea}
}

struct Enzyme<'a> {
    entity: Entity<'a>,
    enzyme_type: char
}
*/