/* TODO
!=use!=f64!=
*!=add!=enzyme!=class!=
*!=add!=random!=spawn!=conditions!=
*!=add!=colision!=physics!=
*!=add!=substrate!=
*!=add!=ability!=for!=substrate!=to!=turn!=into!=product!=through!=enzyme!=
*!=competetive!=inhibitors!=
*!=non!=competetive!=inhibitors!=
*!=use!=constant!=radius!=
* add temp
* add pH
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
const PRODUCT_A_BMP_PATH: &str = "images/A/productA.bmp";
const PRODUCT_B_BTMP_PATH: &str = "images/B/productB.bmp";
const NON_COMP_A_BMP_PATH: &str = "images/A/noncompA.bmp";
const NON_COMP_B_BMP_PATH: &str = "images/B/noncompB.bmp";
const COMP_A_BMP_PATH: &str = "images/A/compA.bmp";
const COMP_B_BMP_PATH: &str = "images/B/compB.bmp";
const ENZYME_A_COMP_BMP_PATH: &str = "images/A/enzymeA_comp.bmp";
const ENZYME_B_COMP_BMP_PATH: &str = "images/B/enzymeB_comp.bmp";
const ENZYME_A_NONCOMP_BMP_PATH: &str = "images/A/enzymeA_noncomp.bmp";
const ENZYME_B_NONCOMP_BMP_PATH: &str = "images/B/enzymeB_noncomp.bmp";

const ADD_ENZYME_A_BUTTON_BMP_PATH: &str = "images/buttons/A/enzyme.bmp";
const ADD_SUBSTRATE_A_BUTTON_BMP_PATH: &str = "images/buttons/A/substrate.bmp";
const ADD_COMP_INHIBITOR_A_BUTTON_BMP_PATH: &str = "images/buttons/A/comp_inhibitor.bmp";
const ADD_NONCOMP_INHIBITOR_A_BUTTON_BMP_PATH: &str = "images/buttons/A/noncomp_inhibitor.bmp";

const ADD_ENZYME_B_BUTTON_BMP_PATH: &str = "images/buttons/B/enzyme.bmp";
const ADD_SUBSTRATE_B_BUTTON_BMP_PATH: &str = "images/buttons/B/substrate.bmp";
const ADD_COMP_INHIBITOR_B_BUTTON_BMP_PATH: &str = "images/buttons/B/comp_inhibitor.bmp";
const ADD_NONCOMP_INHIBITOR_B_BUTTON_BMP_PATH: &str = "images/buttons/B/noncomp_inhibitor.bmp";


const PRODUCT_SPREAD: f64 = std::f64::consts::PI / 8.0;

const ENZYME_RADIUS: f64 = 32.0;
const SUBSTRATE_RADIUS: f64 = 16.0;
const INHIBITOR_RADIUS: f64 = SUBSTRATE_RADIUS;

const BUTTON_WIDTH: u32 = 200;
const BUTTON_HEIGHT: u32 = 70;
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

    // initialize texture creator used for ALL textures
    let texture_creator = canvas.texture_creator();

    // initialize macro molecule vectors
    let mut enzymes: Vec<Enzyme<'_>> = Vec::new();
    let mut substrate: Vec<Substrate<'_>> = Vec::new();
    let mut products: Vec<Entity<'_>> = Vec::new();
    let mut inhibitors: Vec<Inhibitor<'_>> = Vec::new();

    // initialize button rectangles and sprites
    let add_enzyme_a_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32,
        (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 1 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_enzyme_a_button_sprite: sdl2::render::Texture<'_> = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ADD_ENZYME_A_BUTTON_BMP_PATH).unwrap())
        .expect("Failed to create texture.");

    let add_enzyme_b_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32,
        (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 2 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_enzyme_b_button_sprite: sdl2::render::Texture<'_> = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ADD_ENZYME_B_BUTTON_BMP_PATH).unwrap())
        .expect("Failed to create texture.");

    let add_substrate_a_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32,
        (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 3 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_substrate_a_button_sprite: sdl2::render::Texture<'_> = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ADD_SUBSTRATE_A_BUTTON_BMP_PATH).unwrap())
        .expect("Failed to create texture.");

    let add_substrate_b_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32,
        (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 4 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_substrate_b_button_sprite: sdl2::render::Texture<'_> = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ADD_SUBSTRATE_B_BUTTON_BMP_PATH).unwrap())
        .expect("Failed to create texture.");

    let add_comp_a_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32,
        (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 5 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_comp_a_button_sprite: sdl2::render::Texture<'_> = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ADD_COMP_INHIBITOR_A_BUTTON_BMP_PATH).unwrap())
        .expect("Failed to create texture.");

    let add_comp_b_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32,
        (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 6 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_comp_b_button_sprite: sdl2::render::Texture<'_> = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ADD_COMP_INHIBITOR_B_BUTTON_BMP_PATH).unwrap())
        .expect("Failed to create texture.");

    let add_noncomp_a_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32,
        (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 7 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_noncomp_a_button_sprite: sdl2::render::Texture<'_> = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ADD_NONCOMP_INHIBITOR_A_BUTTON_BMP_PATH).unwrap())
        .expect("Failed to create texture.");

    let add_noncomp_b_button = sdl2::rect::Rect::new(WIDTH as i32 - (BUTTON_WIDTH + BUTTON_PADDING) as i32,
        (BUTTON_PADDING + BUTTON_HEIGHT) as i32 * 8 - BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
    let add_noncomp_b_button_sprite: sdl2::render::Texture<'_> = texture_creator
        .create_texture_from_surface(sdl2::surface::Surface::load_bmp(ADD_NONCOMP_INHIBITOR_B_BUTTON_BMP_PATH).unwrap())
        .expect("Failed to create texture.");

    // set background color
    canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 20, 20));
    'game_loop: loop {
        canvas.clear();
        
        // check for collisions amount the enzymes and handle them
        for i in 0..enzymes.len() {
            for j in i + 1..enzymes.len() {
                let r_sq = (enzymes[i].entity.position.0 - enzymes[j].entity.position.0).powi(2)
                                +(enzymes[i].entity.position.1 - enzymes[j].entity.position.1).powi(2);
                if r_sq <= (ENZYME_RADIUS * 2.0).powi(2) {
                    collide(&mut enzymes, i, j, r_sq.sqrt());
                }
            }
        }

        // release substrate from enzyme
        for e in &mut enzymes {
            e.entity.update();
            e.entity.display(&mut canvas);
            if e.status == EnzymeStatus::Complex {
                let rand_num = rand::thread_rng().gen_range(1..=500);
                if rand_num == 100 {
                    e.release_product(&texture_creator, &mut products);
                }
            }
        }

        for i in (0..substrate.len()).rev() {
            // update and display substrate
            substrate[i].entity.update();
            substrate[i].entity.display(&mut canvas);

            let mut to_remove: Vec<usize> = Vec::new(); // vector of indicies to remove

            for j in 0..enzymes.len() {
                // calulate the min distance to be colliding and the distance between enzyme and substrate
                let sq_min_distance = (SUBSTRATE_RADIUS + ENZYME_RADIUS).powi(2);
                let sq_distance = (substrate[i].entity.position.0 - enzymes[j].entity.position.0).powi(2) + (substrate[i].entity.position.1 - enzymes[j].entity.position.1).powi(2);

                // check if the enzyme and substrate are collding
                if sq_distance <= sq_min_distance {

                    // check if the substrate is NOT qualified to be grabed by the enzyme
                    if enzymes[j].status != EnzymeStatus::Natural ||
                    enzymes[j].kind != substrate[i].kind {
                        // reverse the substrate velocity
                        substrate[i].entity.velocity.0 *= -1.0;
                        substrate[i].entity.velocity.1 *= -1.0;
                        let r = sq_distance.sqrt();
                        let delta = SUBSTRATE_RADIUS + ENZYME_RADIUS - r;
                        substrate[i].entity.position.0 += (substrate[i].entity.position.0 - enzymes[j].entity.position.0) / r * delta * 2.0;
                        substrate[i].entity.position.1 += (substrate[i].entity.position.1 - enzymes[j].entity.position.1) / r * delta * 2.0;
                        // have random change to knock out inhibitor if it is competetive
                        if enzymes[j].status == EnzymeStatus::Competetive {
                            if rand::thread_rng().gen_range(1..=10) == 1 {
                                enzymes[j].release_noncomp_inhibitor(&mut inhibitors, &texture_creator)
                            }
                        }
                    } else {
                        // grab the enzyme
                        enzymes[j].grab_substrate(&texture_creator);
                        to_remove.push(i);
                        break;
                    }
                }
            }
            // remove all substrate that were grabed by enzymes and refit the vector
            for index in to_remove {
                substrate.remove(index);
            }
            substrate.shrink_to_fit();
        }

        // update and display all competetive inhibitors
        for i in (0..inhibitors.len()).rev() {
            inhibitors[i].entity.update();
            inhibitors[i].entity.display(&mut canvas);

            let mut to_remove: Vec<usize> = Vec::new(); // vector of indicies to remove

            for j in 0..enzymes.len() {
                // calulate the min distance to be colliding and the distance between enzyme and substrate
                let sq_min_distance = (INHIBITOR_RADIUS + ENZYME_RADIUS).powi(2);
                let sq_distance = (inhibitors[i].entity.position.0 - enzymes[j].entity.position.0).powi(2) + (inhibitors[i].entity.position.1 - enzymes[j].entity.position.1).powi(2);

                // check if the enzyme and substrate are collding
                if sq_distance <= sq_min_distance {
                    // check if the substrate is NOT qualified to be grabed by the enzyme
                    if enzymes[j].status != EnzymeStatus::Natural ||
                    enzymes[j].kind != inhibitors[i].kind {
                        // reverse the substrate velocity
                        inhibitors[i].entity.velocity.0 *= -1.0;
                        inhibitors[i].entity.velocity.1 *= -1.0;
                        let r = sq_distance.sqrt();
                        let delta = INHIBITOR_RADIUS + ENZYME_RADIUS - r;
                        inhibitors[i].entity.position.0 += (inhibitors[i].entity.position.0 - enzymes[j].entity.position.0) / r * delta * 2.0;
                        inhibitors[i].entity.position.1 += (inhibitors[i].entity.position.1 - enzymes[j].entity.position.1) / r * delta * 2.0;
                    } else {
                        // grab the enzyme
                        enzymes[j].grab_inhibitor(inhibitors[i].inhibitor_type.clone(), &texture_creator);
                        to_remove.push(i);
                        break;
                    }
                }
            }
            // remove any inhibitors that have been grabed
            for index in to_remove {
                inhibitors.remove(index);
            }
            inhibitors.shrink_to_fit();
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

        // draw all the the buttons
        canvas.copy(&add_enzyme_a_button_sprite, None,add_enzyme_a_button).unwrap();
        canvas.copy(&add_enzyme_b_button_sprite, None,add_enzyme_b_button).unwrap();
        canvas.copy(&add_substrate_a_button_sprite, None,add_substrate_a_button).unwrap();
        canvas.copy(&add_substrate_b_button_sprite, None,add_substrate_b_button).unwrap();
        canvas.copy(&add_comp_a_button_sprite, None,add_comp_a_button).unwrap();
        canvas.copy(&add_comp_b_button_sprite, None,add_comp_b_button).unwrap();
        canvas.copy(&add_noncomp_a_button_sprite, None,add_noncomp_a_button).unwrap();
        canvas.copy(&add_noncomp_b_button_sprite, None,add_noncomp_b_button).unwrap();

        canvas.present();  // reveal the screen

        let mut event_pump = context.event_pump().expect("Failed to poll events."); // poll for user input
        // handle user input
        for event in event_pump.poll_iter() {
            match event {
                // check if the user wants to quit
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'game_loop;
                }
                // check if the user clicked
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {

                        let cursor = Point::new(x, y); // get cursor position
                        // check which button was clicked based on the cursor position and take the approprate action
                        if add_enzyme_a_button.contains_point(cursor) {
                            let new_enzyme = Enzyme::new(Kind::A, &texture_creator);
                            enzymes.push(new_enzyme);
                        }
                        else if add_enzyme_b_button.contains_point(cursor) {
                            let new_enzyme = Enzyme::new(Kind::B, &texture_creator);
                            enzymes.push(new_enzyme);
                        }
                        else if add_substrate_a_button.contains_point(cursor) {
                            let new_substrate = Substrate::new(Kind::A, &texture_creator);
                            substrate.push(new_substrate);
                        }
                        else if add_substrate_b_button.contains_point(cursor) {
                            let new_substrate = Substrate::new(Kind::B, &texture_creator);
                            substrate.push(new_substrate);
                        }
                        else if add_comp_a_button.contains_point(cursor) {
                            let new_inhibitor = Inhibitor::new(Kind::A, InhibitorType::Competetive, &texture_creator);
                            inhibitors.push(new_inhibitor);
                        }
                        else if add_comp_b_button.contains_point(cursor) {
                            let new_inhibitor = Inhibitor::new(Kind::B, InhibitorType::Competetive, &texture_creator);
                            inhibitors.push(new_inhibitor);
                        }
                        else if add_noncomp_a_button.contains_point(cursor) {
                            let new_inhibitor = Inhibitor::new(Kind::A, InhibitorType::NonCompetetive, &texture_creator);
                            inhibitors.push(new_inhibitor);
                        }
                        else if add_noncomp_b_button.contains_point(cursor) {
                            let new_inhibitor = Inhibitor::new(Kind::B, InhibitorType::NonCompetetive, &texture_creator);
                            inhibitors.push(new_inhibitor);
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
    let delta = ENZYME_RADIUS * 2.0 - r;
    let delta_x = (enzymes[j].entity.position.0 - enzymes[i].entity.position.0) / (2.0 * r) * delta;
    let delta_y = (enzymes[j].entity.position.1 - enzymes[i].entity.position.1) / (2.0 * r) * delta;

    enzymes[i].entity.position.0 -= delta_x;
    enzymes[i].entity.position.1 -= delta_y;

    enzymes[j].entity.position.0 += delta_x;
    enzymes[j].entity.position.1 += delta_y;
    
}

// trait for any class with the entity field

#[derive(PartialEq, Clone)]
enum Kind {
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
    kind: Kind,
    status: EnzymeStatus
}

impl<'a> Enzyme<'a> {
    fn new(
        kind: Kind,
        texture_creator: &'a sdl2::render::TextureCreator<WindowContext>
    ) -> Enzyme<'a> {
        let photo_path: &str;
        match kind {
            Kind::A => {photo_path = ENZYME_A_BMP_PATH}
            Kind::B => {photo_path = ENZYME_B_BMP_PATH}
        }

        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");

        Enzyme { entity: Entity::spawn(sprite, ENZYME_RADIUS, 1.0), kind, status: EnzymeStatus::Natural}

    }
    fn grab_substrate(&mut self, texture_creator: &'a sdl2::render::TextureCreator<WindowContext>) {
        self.status = EnzymeStatus::Complex;

        let photo_path: &str;
        match self.kind {
            Kind::A => {photo_path = COMPLEX_A_BMP_PATH}
            Kind::B => {photo_path = COMPLEX_B_BMP_PATH}
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
        match self.kind {
            Kind::A => {photo_path = ENZYME_A_BMP_PATH}
            Kind::B => {photo_path = ENZYME_B_BMP_PATH}
        }
        let mut sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");
        self.entity.sprite = sprite;

        // load the sprite for the new product
        match self.kind {
            Kind::A => {photo_path = PRODUCT_A_BMP_PATH}
            Kind::B => {photo_path = PRODUCT_B_BTMP_PATH}
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
    fn grab_inhibitor(
        &mut self, inhibitor_type: InhibitorType,
        texture_creator: &'a sdl2::render::TextureCreator<WindowContext>
    ) {
        if inhibitor_type == InhibitorType::Competetive {
            self.status = EnzymeStatus::Competetive;
        } else {
            self.status = EnzymeStatus::NoneCompetetive;
        }

        let photo_path = match self.kind {
            Kind::A => {
                match inhibitor_type {
                    InhibitorType::Competetive => ENZYME_A_COMP_BMP_PATH,
                    InhibitorType::NonCompetetive => ENZYME_A_NONCOMP_BMP_PATH,
                }
            }
            Kind::B => {
                match inhibitor_type {
                    InhibitorType::Competetive => ENZYME_B_COMP_BMP_PATH,
                    InhibitorType::NonCompetetive => ENZYME_B_NONCOMP_BMP_PATH,
                }
            }
        };

        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");
        self.entity.sprite = sprite;
    }

    fn release_noncomp_inhibitor(
        &mut self, inhibitors: &mut Vec<Inhibitor<'a>>,
        texture_creator: &'a sdl2::render::TextureCreator<WindowContext>
    ) {
        let mut new_inhibitor = Inhibitor::new(self.kind.clone(), InhibitorType::Competetive, texture_creator);
        let new_velocity = (self.entity.velocity.0 * 1.5, self.entity.velocity.1 * 1.5);
        let new_position = (self.entity.velocity.0 * (ENZYME_RADIUS + INHIBITOR_RADIUS) + self.entity.position.0,
            self.entity.velocity.1 * (ENZYME_RADIUS + INHIBITOR_RADIUS) + self.entity.position.1);
        new_inhibitor.entity.velocity = new_velocity;
        new_inhibitor.entity.position = new_position;

        inhibitors.push(new_inhibitor);

        self.status = EnzymeStatus::Natural;

        let photo_path: &str;
        match self.kind {
            Kind::A => {photo_path = ENZYME_A_BMP_PATH}
            Kind::B => {photo_path = ENZYME_B_BMP_PATH}
        }
        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");
        self.entity.sprite = sprite;
    }
}

struct Substrate<'a> {
    entity: Entity<'a>,
    kind: Kind,
}
impl<'a> Substrate<'a> {
    fn new(
        kind: Kind,
        texture_creator: &'a sdl2::render::TextureCreator<WindowContext>
    ) -> Substrate<'a> {

        let photo_path = match kind {
            Kind::A => {SUBSTRATE_A_BMP_PATH}
            Kind::B => {SUBSTRATE_B_BMP_PATH}
        };

        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");

        Substrate {entity: Entity::spawn(sprite, SUBSTRATE_RADIUS, 1.5), kind}
    }
}

#[derive(PartialEq, Clone)]
enum InhibitorType {
    Competetive,
    NonCompetetive,
}

struct Inhibitor<'a> {
    entity: Entity<'a>,
    kind: Kind,
    inhibitor_type: InhibitorType,
}
impl<'a> Inhibitor<'a> {
    fn new(
        kind: Kind, inhibitor_type: InhibitorType,
        texture_creator: &'a sdl2::render::TextureCreator<WindowContext>
    ) -> Inhibitor {

        let photo_path = match kind {
            Kind::A => {
                match inhibitor_type {
                    InhibitorType::NonCompetetive => NON_COMP_A_BMP_PATH,
                    InhibitorType::Competetive => COMP_A_BMP_PATH,
                }
            }
            Kind::B => {
                match inhibitor_type {
                    InhibitorType::NonCompetetive => NON_COMP_B_BMP_PATH,
                    InhibitorType::Competetive => COMP_B_BMP_PATH,
                }
            }
        };
        let sprite: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(sdl2::surface::Surface::load_bmp(photo_path).unwrap())
            .expect("Failed to create texture.");

        Inhibitor {entity: Entity::spawn(sprite, INHIBITOR_RADIUS, 1.5), kind, inhibitor_type}
    }
}
