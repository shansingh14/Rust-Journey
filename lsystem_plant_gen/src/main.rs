use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;
use std::time::{Duration, Instant};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const N: usize = 6;
const ANGLE: f32 = 25.0f32.to_radians();


fn main() -> minifb::Result<()> {
    let mut window: Window = Window::new(
        "L-System Generator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;
    let mut buffer: Vec<u32> = vec![0xFFFFFFFF; WIDTH * HEIGHT];

    let rules = [
        Rule {
            from: 'X',
            to: "F+[[X]-X]-F[-FX]+X".into(),
        },
        Rule {
            from: 'F',
            to: "FF".into(),
        },
    ];
    let seq: String = gen_sequence("+++X", &rules, N);
    let commands: Vec<char> = seq.chars().collect();

    // specifying drawing specs
    let mut actions = HashMap::new();
    actions.insert('X', Action::Idle);
    actions.insert('F', Action::Move);
    actions.insert('+', Action::Turn(-ANGLE));
    actions.insert('-', Action::Turn(ANGLE));
    actions.insert('[', Action::Push(0.0));
    actions.insert(']', Action::Pop(0.0));
    let spec: DrawSpec = DrawSpec { actions };

    let mut current = 0;
    let mut last = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last.elapsed() >= Duration::from_millis(5) {
            current += 1;
            last = Instant::now();
        }
        draw_path(&mut buffer, &commands, current, &spec);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }

    Ok(())
}


/* CORE logic functions */
struct Rule {
    from: char,
    to: String,
}

fn create_rule_map(rules: &[Rule]) -> HashMap<char, String> {
    rules
        .iter()
        .map(|r: &Rule| (r.from, r.to.clone()))
        .collect()
}

fn gen_sequence(axiom: &str, rules: &[Rule], n: usize) -> String {
    let rule_map: HashMap<char, String> = create_rule_map(rules);
    let mut cur_seq: String = axiom.to_string();

    for _ in 0..n {
        cur_seq = expand_once(&cur_seq, &rule_map);
    }

    return cur_seq;
}

fn expand_once(axiom: &str, rule_map: &HashMap<char, String>) -> String {
    let mut stem: String = String::new();
    for chr in axiom.to_string().chars() {
        if let Some(rep) = rule_map.get(&chr) {
            stem.push_str(rep);
        } else {
            stem.push(chr);
        }
    }

    return stem;
}

/* Drawing logic - makes everything more difficult */
enum Action {
    Move,
    Turn(f32),
    Pop(f32),
    Push(f32),
    Idle,
}

struct DrawSpec {
    actions: HashMap<char, Action>,
}

struct Pen {
    x: f32,
    y: f32,
    angle: f32,
    stack: Vec<PenState>,
}

struct PenState {
    x: f32,
    y: f32,
    angle: f32,
}

impl Pen {
    fn forward(&mut self, dist: f32, buf: &mut [u32]) {
        let dx: f32 = self.angle.cos() * dist;
        let dy: f32 = self.angle.sin() * dist;
        let x2: f32 = self.x + dx;
        let y2: f32 = self.y + dy;
        draw_line(self.x, self.y, x2, y2, buf);
        self.x = x2;
        self.y = y2;
    }

    fn turn(&mut self, delta: f32) {
        self.angle = (self.angle + delta) % (2.0 * std::f32::consts::PI);
    }

    fn push_state(&mut self) {
        self.stack.push(PenState {
            x: self.x,
            y: self.y,
            angle: self.angle,
        });
    }

    fn pop_state(&mut self) {
        if let Some(state) = self.stack.pop() {
            self.x = state.x;
            self.y = state.y;
            self.angle = state.angle;
        }
    }
}

fn draw_line(x0: f32, y0: f32, x1: f32, y1: f32, buf: &mut [u32]) {
    let mut x0: i32 = x0.round() as i32;
    let mut y0: i32 = y0.round() as i32;
    let x1: i32 = x1.round() as i32;
    let y1: i32 = y1.round() as i32;

    let dx: i32 = (x1 - x0).abs();
    let dy: i32 = (y1 - y0).abs();
    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };
    let mut err: i32 = dx - dy;

    loop {
        if (0..WIDTH as i32).contains(&x0) && (0..HEIGHT as i32).contains(&y0) {
            buf[(y0 as usize) * WIDTH + (x0 as usize)] = 0xFF_000000;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2: i32 = err * 2;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_path(buf: &mut [u32], commands: &[char], upto: usize, spec: &DrawSpec) {
    for px in buf.iter_mut() {
        *px = 0xFFFFFFFF;
    }

    let mut x: f32 = 0.0f32;
    let mut y: f32 = 0.0f32;
    let mut heading: f32 = 0.0f32;
    let mut minx: f32 = x;
    let mut maxx: f32 = x;
    let mut miny: f32 = y;
    let mut maxy: f32 = y;

    for &ch in commands {
        match spec.actions.get(&ch) {
            Some(Action::Move) => {
                x += heading.cos();
                y += heading.sin();
                minx = minx.min(x);
                maxx = maxx.max(x);
                miny = miny.min(y);
                maxy = maxy.max(y);
            }
            Some(Action::Turn(d)) => heading += *d,
            _ => {}
        }
    }

    let path_w: f32 = maxx - minx;
    let path_h: f32 = maxy - miny;
    let margin: f32 = 0.05f32;
    let avail_w: f32 = WIDTH as f32 * (1.0 - 2.0 * margin);
    let avail_h: f32 = HEIGHT as f32 * (1.0 - 2.0 * margin);
    let scale: f32 = (avail_w / path_w).min(avail_h / path_h);
    let start_x: f32 = margin * WIDTH as f32 - minx * scale;
    let start_y: f32 = margin * HEIGHT as f32 - miny * scale;
    let start_stack: Vec<PenState> = Vec::new();

    let mut pen: Pen = Pen {
        x: start_x,
        y: start_y,
        angle: 0.0,
        stack: start_stack,
    };
    for &ch in &commands[..upto.min(commands.len())] {
        if let Some(act) = spec.actions.get(&ch) {
            match *act {
                Action::Move => pen.forward(scale, buf),
                Action::Turn(d) => pen.turn(d),
                Action::Push(d) => {
                    pen.push_state();
                    pen.turn(d)
                }
                Action::Pop(d) => {
                    pen.pop_state();
                    pen.turn(d);
                }
                Action::Idle => {}
            }
        }
    }
}


