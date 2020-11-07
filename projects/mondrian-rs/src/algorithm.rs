use rand::{distributions::Distribution, Rng};
use rand_distr::{Beta, WeightedIndex};
use svg::{Document, Node};

#[derive(Debug)]
pub struct Rectangle {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

const SIZE: f32 = 100.0;
const WIDTH: f32 = 1.6;

impl Default for Rectangle {
    fn default() -> Self {
        Self { x1: 0.0, y1: 0.0, x2: WIDTH * SIZE, y2: 1.0 * SIZE }
    }
}

pub fn split(input: Vec<Rectangle>) -> Vec<Rectangle> {
    let mut out = vec![];
    let beta = Beta::new(10.0, 10.0).unwrap();
    let rng = &mut rand::thread_rng();
    for Rectangle { x1, y1, x2, y2 } in input {
        let t = beta.sample(rng);
        let r = rng.gen_range(0.0, 1.0);
        if r < 0.3 * (x2 - x1) / (y2 - y1) {
            out.push(Rectangle { x1, y1, x2: x1 + (x2 - x1) * t, y2 });
            out.push(Rectangle { x1: x1 + (x2 - x1) * t, y1, x2, y2 });
        }
        else if 1.0 - r < 0.5 * (y2 - y1) / (x2 - x1) {
            out.push(Rectangle { x1, y1, x2, y2: y1 + (y2 - y1) * t });
            out.push(Rectangle { x1, y1: y1 + (y2 - y1) * t, x2, y2 });
        }
        else {
            out.push(Rectangle { x1, y1, x2, y2 })
        }
    }
    return out;
}

pub fn rectangle_round(input: Vec<Rectangle>, round: f32) -> Vec<Rectangle> {
    let round = round * SIZE;
    let mut out = vec![];
    for Rectangle { x1, y1, x2, y2 } in input {
        out.push(Rectangle {
            x1: (x1 *round).round() / round,
            y1: (y1 *round).round() / round,
            x2: (x2 *round).round() / round,
            y2: (y2 *round).round() / round,
        })
    }
    return out;

}

const COLORS: [&str; 5] = ["#E6E6E6", "#0D0D0D", "#CC1A1A", "#1A1A80", "#E6B21A"];
const WEIGHT: [i32; 5] = [1, 1, 3, 3, 3];

fn svg_rectangle(x1: f32, y1: f32, x2: f32, y2: f32) -> svg::node::element::Rectangle {
    let dist = WeightedIndex::new(&WEIGHT).unwrap();
    let color = COLORS[dist.sample(&mut rand::thread_rng())];
    svg::node::element::Rectangle::new()
        .set("x", x1)
        .set("y", y1)
        .set("width", x2 - x1)
        .set("height", y2 - y1)
        .set("fill", color)
}

fn svg_line(x1: f32, y1: f32, x2: f32, y2: f32, line: f32) -> svg::node::element::Line {
    svg::node::element::Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("style", format!("stroke:black;stroke-width:{}", line))
}

pub fn rectangle_svg(input: Vec<Rectangle>, line: f32) -> Document {
    let mut document = Document::new().set("viewBox", (0, 0, WIDTH * SIZE, 1.0 * SIZE));
    for Rectangle { x1, y1, x2, y2 } in input {
        document.append(svg_rectangle(x1, y1, x2, y2));
        document.append(svg_line(x1, y2, x2, y2,line));
        document.append(svg_line(x1, y1, x2, y1,line));
        document.append(svg_line(x1, y1, x1, y2,line));
        document.append(svg_line(x2, y1, x2, y2,line));
    }
    return document;
}
