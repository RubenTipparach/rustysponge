extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    // initialize player
    x: f64,
    y: f64,

    //initialize pong
    px: f64,
    py: f64,

    //init enemy
    ex: f64,
    ey: f64,
}

impl App {
    // kickass render loop
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND_BLUE: [f32; 4] = [0.0, 0.0, 0.1, 1.0];

        const GREEN: [f32; 4] = [0.0, 0.5, 0.2, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        const WHITE:   [f32; 4] = [0.8, 0.8, 0.8, 1.0];
        

        let square = rectangle::square(0.0, 0.0, 50.0);

        let pongBall = rectangle::square(0.0, 0.0, 10.0);

        // unused
        let rotation = self.rotation;
        let (x, y, ex, ey, px, py) = (
            ((args.width / 2 ) as f64) + self.x,
            ((args.height / 2) as f64) + self.y,
            ((args.width / 2 ) as f64) + self.ex,
            ((args.height / 2) as f64) + self.ey,
            ((args.width / 2 ) as f64) + self.px,
            ((args.height / 2) as f64) + self.py);        

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND_BLUE, gl);

            let playerTransform = c.transform.trans(x , y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, playerTransform, gl);

            let enemyTransform = c.transform.trans(ex , ey)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // porbably need to draw other guy too
            rectangle(GREEN, square, enemyTransform, gl);


           let pongTransform = c.transform.trans(px , py)
                                       .rot_rad(rotation)
                                       .trans(-5.0, -5.0);

            // porbably need to draw other guy too
            rectangle(WHITE, pongBall, pongTransform, gl);
        });
    }

    // kickass update loop
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        //self.rotation += 2.0 * args.dt;
    }

    fn move_x(&mut self, val: f64)
    {
        self.x += val;
    }

    fn move_y(&mut self, val: f64)
    {
        self.y += val;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [1280, 720]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,

        //initialize player
        x: 500.0,
        y: 0.0,

        //initialize pong
        px: 0.0,
        py: 0.0,

        //init enemy
        ex: -500.0,
        ey: 0.0,

    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        let speed = 10.0f64;

        // keyboard functions! yay
        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            //app.move_x(-1.0 * speed);
        }

        if let Some(Button::Keyboard(Key::D)) = e.press_args() {
            //app.move_x(1.0 * speed);
        }

        if let Some(Button::Keyboard(Key::W)) = e.press_args() {
            app.move_y(-1.0 * speed);
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            app.move_y(1.0 * speed);
        }
    }
}
