/* TODO
!=use!=f64!=
~ add enzyme class
*!=add!=random!=spawn!=conditions!=
*!=add!=colision!=physics!=
* add substrate
* add ability for substrate to turn into product through enzyme
* competetive inhibitors
* non-competetive inhibitors
* add nature reaction and subatomic repel
*/


extern crate sdl2;
extern crate rand;

use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::rect::Point;
use sdl2::video::WindowContext;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const ENZYME_A_BMP_PATH: &str = "enzymeA.bmp";
const ENZYME_RADIUS: f64 = 50.0;

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

    let mut enzymes = vec![Enzyme::new(EnzymeType::A, &texture_creator)];

    let add_enzyme_a_button = sdl2::rect::Rect::new(WIDTH as i32 - 125, 25, 100, 50);

    'game_loop: loop {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        
        for i in 0..enzymes.len() {
            for j in i + 1..enzymes.len() {
                let r_sq = (enzymes[i].entity.position.0 - enzymes[j].entity.position.0).powi(2)
                                +(enzymes[i].entity.position.1 - enzymes[j].entity.position.1).powi(2);
                if r_sq <= (enzymes[i].entity.radius + enzymes[j].entity.radius).powi(2) {
                    collide(&mut enzymes, i, j, r_sq.sqrt());
                }
            }
        }
        for e in &mut enzymes {
            e.entity.update();
            e.entity.display(&mut canvas);
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(128, 0, 0));
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
                            let new_enzyme = Enzyme::new(EnzymeType::A, &texture_creator);
                            enzymes.push(new_enzyme);
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
    radius: f64,
    position: (f64, f64),
    velocity: (f64, f64),
    rect: sdl2::rect::Rect,
}

impl<'a> Entity<'a> {
    pub fn spawn(sprite: sdl2::render::Texture<'a>, radius: f64, velocity: f64) -> Entity {
        let t = rand::thread_rng().gen_range(0.0..=2.0 * std::f64::consts::PI);
        Entity::new(sprite, radius, (WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0), (t.cos() * velocity, t.sin() * velocity))
    }
    pub fn new(sprite: sdl2::render::Texture<'a>, radius: f64, position: (f64, f64), velocity: (f64, f64)) -> Entity{
        let rect = sdl2::rect::Rect::new((position.0 - radius).round() as i32, (position.1 + radius).round() as i32,
            radius as u32 * 2, radius as u32* 2);
        Entity {sprite, radius, position, velocity, rect}
    }
    pub fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.bounds_check();
    }
    fn bounds_check(& mut self) {
        // check x bounds
        if (self.position.0 - self.radius <= 0.0 && self.velocity.0 < 0.0) || (self.position.0 + self.radius >= WIDTH as f64 && self.velocity.0 > 0.0) {
            self.velocity.0 *= -1.0;
        }
        // check y bounds
        if (self.position.1 - self.radius <= 0.0) || (self.position.1 + self.radius >= HEIGHT as f64) {
            self.velocity.1 *= -1.0;
        }
    }
    pub fn display(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.rect.set_x((self.position.0 - self.radius).round() as i32);
        self.rect.set_y((self.position.1 - self.radius).round() as i32);
        canvas.copy(&self.sprite, None, self.rect).expect("Failed to display entity.");
    }
}

fn collide(enzymes: &mut Vec<Enzyme>, i: usize, j: usize, r: f64) {
    // beta is the angle that the line which is tangential to both circles makes with the x-axis
    let beta = ((enzymes[j].entity.position.1 - enzymes[i].entity.position.1)/(enzymes[j].entity.position.0 - enzymes[i].entity.position.0)).atan() + std::f64::consts::PI/2.0;

    // v_theta_i is the angle of ei's velocity
    let v_theta_1 = (enzymes[i].entity.velocity.1 / enzymes[i].entity.velocity.0).atan();
    let v_theta_2 = (enzymes[j].entity.velocity.1 / enzymes[j].entity.velocity.0).atan();

    // dt_i is the change in angle of velocity of ei caused by the collision.
    let dt1 = 2.0 * (beta - v_theta_1);
    let dt2 = 2.0 * (beta - v_theta_2);

    //rotate enzymes[i].entity and enzymes[j].entity velocity by the appropriate amount
    let new_x = enzymes[i].entity.velocity.0 * dt1.cos() - enzymes[i].entity.velocity.1 * dt1.sin();
    let new_y = enzymes[i].entity.velocity.0 * dt1.sin() + enzymes[i].entity.velocity.1 * dt1.cos();
    enzymes[i].entity.velocity = (new_x, new_y);

    let new_x = enzymes[j].entity.velocity.0 * dt2.cos() - enzymes[j].entity.velocity.1 * dt2.sin();
    let new_y = enzymes[j].entity.velocity.0 * dt2.sin() + enzymes[j].entity.velocity.1 * dt2.cos();
    enzymes[j].entity.velocity = (new_x, new_y);

    // make sure that the entities are not overlaping
    let delta = enzymes[i].entity.radius + enzymes[j].entity.radius - r;
    let delta_x = (enzymes[j].entity.position.0 - enzymes[i].entity.position.0) / (2.0 * r) * delta;
    let delta_y = (enzymes[j].entity.position.1 - enzymes[i].entity.position.1) / (2.0 * r) * delta;

    enzymes[i].entity.position.0 -= delta_x;
    enzymes[i].entity.position.1 -= delta_y;

    enzymes[j].entity.position.0 += delta_x;
    enzymes[j].entity.position.1 += delta_y;
    
}

// trait for any class with the entity field

enum EnzymeType {
    A,
}

struct Enzyme<'a> {
    entity: Entity<'a>,
    enzyme_type: EnzymeType,
}

impl<'a> Enzyme<'a> {
    fn new(
        enzyme_type: EnzymeType,
        texture_creator: &'a sdl2::render::TextureCreator<WindowContext>
    ) -> Enzyme<'a> {

        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ENZYME_A_BMP_PATH).unwrap())
            .expect("Failed to create texture.");

        Enzyme { entity: Entity::spawn(sprite, ENZYME_RADIUS, 1.0), enzyme_type}

    }
}
