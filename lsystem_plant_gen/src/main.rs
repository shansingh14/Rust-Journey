use std::collections::HashMap;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize  = 800;
const HEIGHT: usize = 600;
const N: usize      = 3;
const ANGLE: f32    = 90.0;

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
    let rule_map = create_rule_map(rules);
    let mut cur_seq = axiom.to_string();

    for i in 0..n {
        cur_seq = expand_once(&cur_seq, &rule_map);
        println!("Gen {} : {}", i, cur_seq);
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


/* Drawing logic/pain */
struct Pen {
    x:     f32,
    y:     f32,
    angle: f32,
}

impl Pen {
    fn forward(&mut self, dist: f32, buf: &mut [u32]) {
        let dx = self.angle.to_radians().cos() * dist;
        let dy = self.angle.to_radians().sin() * dist;
        let x2 = self.x + dx;
        let y2 = self.y + dy;
        draw_line(self.x, self.y, x2, y2, buf);
        self.x = x2;
        self.y = y2;
    }

    fn turn(&mut self, delta: f32) {
        self.angle = (self.angle + delta) % 360.0;
    }
}

fn draw_line(x0: f32, y0: f32, x1: f32, y1: f32, buf: &mut [u32]) {
    let mut x0 = x0.round() as i32;
    let mut y0 = y0.round() as i32;
    let x1 = x1.round() as i32;
    let y1 = y1.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    while x0 != x1 || y0 != y1 {
        if x0 >= 0 && x0 < WIDTH as i32 && y0 >= 0 && y0 < HEIGHT as i32 {
            buf[(y0 as usize) * WIDTH + x0 as usize] = 0xFF_000000;
        }
        let e2 = err * 2;
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

fn draw_sierpinski(
    buffer: &mut [u32],
    commands: &[char],
    upto: usize,
    start: (f32, f32),
    step: f32,
) {
    for pixel in buffer.iter_mut() {
        *pixel = 0xFFFFFFFF;
    }

    let mut pen = Pen { x: start.0, y: start.1, angle: 0.0 };
    for &ch in &commands[..upto.min(commands.len())] {
        match ch {
            'F' | 'G' => pen.forward(step, buffer),
            '+'       => pen.turn(-ANGLE),
            '-'       => pen.turn(ANGLE),
            _         => {}
        }
    }
}

fn main() -> minifb::Result<()> {
    //Setup window & buffer
    let mut window = Window::new(
        "Sierpinski",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;
    let mut buffer = vec![0xFFFFFFFF; WIDTH * HEIGHT];

    // Precompute L‑system & geometry
    let rules = [
        Rule { from: 'F', to: "F+F-F-F+F".into() },
    ];
    let seq = gen_sequence("F-G-G", &rules, N);
    let commands: Vec<char> = seq.chars().collect();

    let base_len = 400.0;
    let levels   = 2f32.powi(N as i32);
    let step     = 20.0;
    let tri_w    = base_len;
    let tri_h    = (3.0_f32.sqrt() / 2.0) * tri_w;
    let start = (
        (WIDTH as f32  - tri_w) / 2.0,
        (HEIGHT as f32 - tri_h) / 2.0,
    );

    let mut current = 0;
    let mut last    = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last.elapsed() >= Duration::from_millis(50) {
            current += 1;
            last = Instant::now();
        }

        // Call your modular draw function
        draw_sierpinski(&mut buffer, &commands, current, (0.0, HEIGHT as f32), step);

        // Present
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }

    Ok(())
}
