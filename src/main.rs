/* TODO
!=use!=f64!=
~ add enzyme class
*!=add!=random!=spawn!=conditions!=
*!=add!=colision!=physics!=
*!=add!=substrate!=
*!=add!=ability!=for!=substrate!=to!=turn!=into!=product!=through!=enzyme!=
* competetive inhibitors
* non-competetive inhibitors
* add temp
* add pH
* make the product rate slow down as the tempurature and pH goes beyond the regular range
    by decreasing the likely hood that enzymes will release products
* add buffers
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

const ENZYME_A_BMP_PATH: &str = "images/A/enzymeA.bmp";
const ENZYME_B_BMP_PATH: &str = "images/B/enzymeB.bmp";
const SUBSTRATE_A_BMP_PATH: &str = "images/A/substrateA.bmp";
const SUBSTRATE_B_BMP_PATH: &str = "images/B/substrateB.bmp";
const COMPLEX_A_BMP_PATH: &str = "images/A/complexA.bmp";
const COMPLEX_B_BMP_PATH: &str = "images/B/complexB.bmp";
const PRODUCT_A_PATH: &str = "images/A/productA.bmp";
const PRODUCT_B_PATH: &str = "images/B/productB.bmp";

const PRODUCT_SPREAD: f64 = std::f64::consts::PI / 8.0;

const ENZYME_RADIUS: f64 = 32.0;
const SUBSTRATE_RADIUS: f64 = 16.0;

const BUTTON_WIDTH: u32 = 200;
const BUTTON_HEIGHT: u32 = 75;
const BUTTON_PADDING: u32 = 15;



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

    let mut enzymes: Vec<Enzyme<'_>> = Vec::new();
    let mut substrate: Vec<Substrate<'_>> = Vec::new();
    let mut products: Vec<Entity<'_>> = Vec::new();

    let add_enzyme_a_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32, (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 1 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_enzyme_b_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32, (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 2 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_substrate_a_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32, (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 3 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_substrate_b_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32, (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 4 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);

    'game_loop: loop {
        // TODO remove
        println!("{}", products.len());

        canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 20, 20));
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
            if e.status == EnzymeStatus::Complex {
                let rand_num = rand::thread_rng().gen_range(1..=100);
                if rand_num == 100 {
                    e.release_product(&texture_creator, &mut products);
                }
            }
        }
        for i in (0..substrate.len()).rev() {
            substrate[i].entity.update();
            substrate[i].entity.display(&mut canvas);
            let mut to_remove: Vec<usize> = Vec::new();
            for j in 0..enzymes.len() {
                let sq_min_distance = (substrate[i].entity.radius + enzymes[j].entity.radius).powi(2);
                let sq_distance = (substrate[i].entity.position.0 - enzymes[j].entity.position.0).powi(2) + (substrate[i].entity.position.1 - enzymes[j].entity.position.1).powi(2);
                if sq_distance <= sq_min_distance {
                    if enzymes[j].status != EnzymeStatus::Natural ||
                    enzymes[j].enzyme_type != substrate[i].substrate_type {
                        substrate[i].entity.velocity.0 *= -1.0;
                        substrate[i].entity.velocity.1 *= -1.0;
                        let r = sq_distance.sqrt();
                        let delta = substrate[i].entity.radius + enzymes[j].entity.radius - r;
                        substrate[i].entity.position.0 += (substrate[i].entity.position.0 - enzymes[j].entity.position.0) / r * delta * 2.0;
                        substrate[i].entity.position.1 += (substrate[i].entity.position.1 - enzymes[j].entity.position.1) / r * delta * 2.0;
                    } else {
                        enzymes[j].grab_substrate(&texture_creator);
                        to_remove.push(i);
                    break;
                    }
                }
            }
            for index in to_remove {
                substrate.remove(index);
            }
            substrate.shrink_to_fit();
        }

        // update all product position and remove any products that are outside of bounds
        {
            let mut to_remove: Vec<usize> = Vec::new();
            for i in (0..products.len()).rev() {
                products[i].position.0 += products[i].velocity.0;
                products[i].position.1 += products[i].velocity.1;
                products[i].display(&mut canvas);
                if products[i].position.1 <= 0.0 || products[i].position.0 >= WIDTH as f64 || products[i].position.1 <= 0.0
                || products[i].position.1 >= HEIGHT as f64 {to_remove.push(i)}
            }
            for index in to_remove {
                products.remove(index);
            }
            products.shrink_to_fit();
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(128, 0, 0));
        canvas.fill_rect(add_enzyme_a_button).expect("Failed to draw rectangle.");
        canvas.fill_rect(add_enzyme_b_button).expect("Failed to draw rectangle.");
        canvas.fill_rect(add_substrate_a_button).expect("Failed to draw rectangle.");
        canvas.fill_rect(add_substrate_b_button).expect("Failed to draw rectangle.");
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
                        let cursor = Point::new(x, y);
                        if add_enzyme_a_button.contains_point(cursor) {
                            let new_enzyme = Enzyme::new(Type::A, &texture_creator);
                            enzymes.push(new_enzyme);
                        }
                        else if add_enzyme_b_button.contains_point(cursor) {
                            let new_enzyme = Enzyme::new(Type::B, &texture_creator);
                            enzymes.push(new_enzyme);
                        }
                        else if add_substrate_a_button.contains_point(cursor) {
                            let new_substrate = Substrate::new(Type::A, &texture_creator);
                            substrate.push(new_substrate);
                        }
                        else if add_substrate_b_button.contains_point(cursor) {
                            let new_substrate = Substrate::new(Type::B, &texture_creator);
                            substrate.push(new_substrate);
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
        if self.position.0 - self.radius <= 0.0 && self.velocity.0 < 0.0 {
            self.velocity.0 *= -1.0;
            self.position.0 = self.radius;
        }
        else if self.position.0 + self.radius >= WIDTH as f64 && self.velocity.0 > 0.0 {
            self.velocity.0 *= -1.0;
            self.position.0 = WIDTH as f64 - self.radius;
        }

        // check y bounds
        if self.position.1 - self.radius <= 0.0 {
            self.velocity.1 *= -1.0;
            self.position.1 = self.radius;
        }
        else if self.position.1 + self.radius >= HEIGHT as f64 {
            self.velocity.1 *= -1.0;
            self.position.1 = HEIGHT as f64 - self.radius;
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

#[derive(PartialEq)]
enum Type {
    A,
    B,
}

#[derive(PartialEq)]
enum EnzymeStatus {
    Natural,
    Complex,
    Denatured,
    Competetive,
    NoneCompetetive,
}

struct Enzyme<'a> {
    entity: Entity<'a>,
    enzyme_type: Type,
    status: EnzymeStatus
}

impl<'a> Enzyme<'a> {
    fn new(
        enzyme_type: Type,
        texture_creator: &'a sdl2::render::TextureCreator<WindowContext>
    ) -> Enzyme<'a> {
        let photo_path: &str;
        match enzyme_type {
            Type::A => {photo_path = ENZYME_A_BMP_PATH}
            Type::B => {photo_path = ENZYME_B_BMP_PATH}
        }

        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");

        Enzyme { entity: Entity::spawn(sprite, ENZYME_RADIUS, 1.0), enzyme_type, status: EnzymeStatus::Natural}

    }
    fn grab_substrate(&mut self, texture_creator: &'a sdl2::render::TextureCreator<WindowContext>) {
        self.status = EnzymeStatus::Complex;

        let photo_path: &str;
        match self.enzyme_type {
            Type::A => {photo_path = COMPLEX_A_BMP_PATH}
            Type::B => {photo_path = COMPLEX_B_BMP_PATH}
        }

        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");

        self.entity.sprite = sprite;
    }
    fn release_product(&mut self, texture_creator: &'a sdl2::render::TextureCreator<WindowContext>, products: &mut  Vec<Entity<'a>>) {
        // change the enzyme sprite back to normal
        self.status = EnzymeStatus::Natural;
        let mut photo_path: &str;
        match self.enzyme_type {
            Type::A => {photo_path = ENZYME_A_BMP_PATH}
            Type::B => {photo_path = ENZYME_B_BMP_PATH}
        }
        let mut sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");
        self.entity.sprite = sprite;

        // load the sprite for the new product
        match self.enzyme_type {
            Type::A => {photo_path = PRODUCT_A_PATH}
            Type::B => {photo_path = PRODUCT_B_PATH}
        }
        
        // calculate new velocity
        let v1 = (
            (self.entity.velocity.0 * PRODUCT_SPREAD.cos() - self.entity.velocity.1 * PRODUCT_SPREAD.sin()) * 2.0,
            (self.entity.velocity.0 * PRODUCT_SPREAD.sin() + self.entity.velocity.1 * PRODUCT_SPREAD.cos()) * 2.0,
        );
        let v2 = (
            (self.entity.velocity.0 * (-1.0 * PRODUCT_SPREAD).cos() - self.entity.velocity.1 * (-1.0 * PRODUCT_SPREAD).sin()) * 2.0,
            (self.entity.velocity.0 * (-1.0 * PRODUCT_SPREAD).sin() + self.entity.velocity.1 * (-1.0 * PRODUCT_SPREAD).cos()) * 2.0,
        );

        sprite = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");
        
        let product1: Entity<'_> = Entity::new(sprite, SUBSTRATE_RADIUS, self.entity.position, v1);

        sprite = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");

        let product2: Entity<'_> = Entity::new(sprite, SUBSTRATE_RADIUS, self.entity.position, v2);

        products.push(product1);
        products.push(product2);

    }
}

struct Substrate<'a> {
    entity: Entity<'a>,
    substrate_type: Type,
}

impl<'a> Substrate<'a> {
    fn new(
        substrate_type: Type,
        texture_creator: &'a sdl2::render::TextureCreator<WindowContext>
    ) -> Substrate<'a> {
        let photo_path: &str;
        match substrate_type {
            Type::A => {photo_path = SUBSTRATE_A_BMP_PATH}
            Type::B => {photo_path = SUBSTRATE_B_BMP_PATH}
        }

        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");

        Substrate {entity: Entity::spawn(sprite, SUBSTRATE_RADIUS, 1.5), substrate_type}
    }
}
