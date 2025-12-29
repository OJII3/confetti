use gtk4::prelude::*;
use gtk4::{gdk, glib, Application, ApplicationWindow, DrawingArea};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use rand::Rng;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::time::Instant;

const APP_ID: &str = "com.github.ojii3.confetti";
const PARTICLE_COUNT: usize = 300;
const DURATION_SECS: f64 = 4.0;
const FRAME_RATE: u32 = 60;

#[derive(Clone)]
struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    width: f64,
    height: f64,
    rotation: f64,
    rotation_speed: f64,
    color: (f64, f64, f64),
    alpha: f64,
}

impl Particle {
    fn new(screen_width: f64, screen_height: f64) -> Self {
        let mut rng = rand::thread_rng();

        let from_left = rng.gen_bool(0.5);
        let x = if from_left { 0.0 } else { screen_width };

        let y = screen_height * rng.gen_range(0.3..0.7);

        let target_x = screen_width / 2.0 + rng.gen_range(-200.0..200.0);
        let target_y = screen_height * rng.gen_range(0.2..0.5);

        let speed = rng.gen_range(800.0..1500.0);
        let dx = target_x - x;
        let dy = target_y - y;
        let dist = (dx * dx + dy * dy).sqrt();
        let vx = (dx / dist) * speed;
        let vy = (dy / dist) * speed - rng.gen_range(200.0..500.0);

        let colors = [
            (1.0, 0.2, 0.3), // Red
            (1.0, 0.5, 0.0), // Orange
            (1.0, 0.9, 0.0), // Yellow
            (0.2, 0.8, 0.2), // Green
            (0.2, 0.6, 1.0), // Blue
            (0.6, 0.2, 0.8), // Purple
            (1.0, 0.4, 0.7), // Pink
            (0.0, 0.9, 0.9), // Cyan
        ];
        let color = colors[rng.gen_range(0..colors.len())];

        Particle {
            x,
            y,
            vx,
            vy,
            width: rng.gen_range(8.0..16.0),
            height: rng.gen_range(12.0..24.0),
            rotation: rng.gen_range(0.0..PI * 2.0),
            rotation_speed: rng.gen_range(-10.0..10.0),
            color,
            alpha: 1.0,
        }
    }

    fn update(&mut self, dt: f64) {
        self.vy += 600.0 * dt; // gravity
        self.vx *= 0.99; // air resistance
        self.vy *= 0.99;

        self.x += self.vx * dt;
        self.y += self.vy * dt;

        self.rotation += self.rotation_speed * dt;
    }

    fn draw(&self, ctx: &gtk4::cairo::Context) {
        ctx.save().unwrap();

        ctx.translate(self.x, self.y);
        ctx.rotate(self.rotation);

        ctx.set_source_rgba(self.color.0, self.color.1, self.color.2, self.alpha);
        ctx.rectangle(
            -self.width / 2.0,
            -self.height / 2.0,
            self.width,
            self.height,
        );
        ctx.fill().unwrap();

        ctx.restore().unwrap();
    }
}

struct ConfettiState {
    particles: Vec<Particle>,
    start_time: Instant,
    last_frame: Instant,
}

impl ConfettiState {
    fn new(width: f64, height: f64) -> Self {
        let particles = (0..PARTICLE_COUNT)
            .map(|_| Particle::new(width, height))
            .collect();

        ConfettiState {
            particles,
            start_time: Instant::now(),
            last_frame: Instant::now(),
        }
    }

    fn update(&mut self) -> bool {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f64();
        self.last_frame = now;

        let elapsed = now.duration_since(self.start_time).as_secs_f64();

        if elapsed > DURATION_SECS {
            return false;
        }

        let fade_start = DURATION_SECS - 1.0;
        let alpha = if elapsed > fade_start {
            1.0 - (elapsed - fade_start)
        } else {
            1.0
        };

        for particle in &mut self.particles {
            particle.update(dt);
            particle.alpha = alpha;
        }

        true
    }

    fn draw(&self, ctx: &gtk4::cairo::Context) {
        ctx.set_operator(gtk4::cairo::Operator::Clear);
        ctx.paint().unwrap();
        ctx.set_operator(gtk4::cairo::Operator::Over);

        for particle in &self.particles {
            particle.draw(ctx);
        }
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let display = gdk::Display::default().expect("Could not get default display");
    let monitors = display.monitors();
    let monitor = monitors
        .item(0)
        .and_then(|m| m.downcast::<gdk::Monitor>().ok())
        .expect("Could not get primary monitor");

    let geometry = monitor.geometry();
    let screen_width = geometry.width() as f64;
    let screen_height = geometry.height() as f64;

    let state = Rc::new(RefCell::new(ConfettiState::new(
        screen_width,
        screen_height,
    )));

    let drawing_area = DrawingArea::new();
    drawing_area.set_content_width(screen_width as i32);
    drawing_area.set_content_height(screen_height as i32);

    let state_draw = Rc::clone(&state);
    drawing_area.set_draw_func(move |_, ctx, _, _| {
        state_draw.borrow().draw(ctx);
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Confetti")
        .default_width(screen_width as i32)
        .default_height(screen_height as i32)
        .decorated(false)
        .child(&drawing_area)
        .build();

    // Initialize layer shell
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::None);

    // Anchor to all edges to cover full screen
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    // Make clicks pass through
    window.set_exclusive_zone(-1);

    // CSS for transparency
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_string(
        "window, window.background {
            background-color: transparent;
        }",
    );

    gtk4::style_context_add_provider_for_display(
        &display,
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let drawing_area_ref = drawing_area.clone();
    let state_update = Rc::clone(&state);
    let window_ref = window.clone();

    glib::timeout_add_local(
        std::time::Duration::from_millis(1000 / FRAME_RATE as u64),
        move || {
            let should_continue = state_update.borrow_mut().update();

            if should_continue {
                drawing_area_ref.queue_draw();
                glib::ControlFlow::Continue
            } else {
                window_ref.close();
                glib::ControlFlow::Break
            }
        },
    );

    window.present();
}
