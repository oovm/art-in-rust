use crate::Mondrian;
use rand::{distributions::Distribution, rngs::StdRng, thread_rng, Rng, SeedableRng};
use rand_distr::{Beta, WeightedIndex};
use svg::{Document, Node};

#[derive(Debug)]
pub struct Rectangle {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { x1: 0.0, y1: 0.0, x2: 1.618, y2: 1.0 }
    }
}

impl Mondrian {
    pub fn generate(&mut self, nest: u32) -> Document {
        let mut v = vec![Rectangle { x1: 0.0, y1: 0.0, x2: self.width * self.size, y2: 1.0 * self.size }];
        for _ in 0..nest {
            v = self.split(v)
        }
        v = self.rectangle_round(v);
        self.rectangle_svg(v)
    }

    pub fn split(&mut self, input: Vec<Rectangle>) -> Vec<Rectangle> {
        let mut out = vec![];
        let beta = Beta::new(10.0, 10.0).unwrap();
        for Rectangle { x1, y1, x2, y2 } in input {
            let t = beta.sample(&mut self.rng);
            let r = self.rng.gen_range(0.0, 1.0);
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

    pub fn rectangle_svg(&mut self, input: Vec<Rectangle>) -> Document {
        let mut document = Document::new().set("viewBox", (0, 0, self.width * self.size, 1.0 * self.size));
        for Rectangle { x1, y1, x2, y2 } in input {
            document.append(self.svg_rectangle(x1, y1, x2, y2));
            document.append(self.svg_line(x1, y2, x2, y2));
            document.append(self.svg_line(x1, y1, x2, y1));
            document.append(self.svg_line(x1, y1, x1, y2));
            document.append(self.svg_line(x2, y1, x2, y2));
        }
        return document;
    }

    fn rectangle_round(&self, input: Vec<Rectangle>) -> Vec<Rectangle> {
        let round = self.grid_round * self.size;
        let mut out = vec![];
        for Rectangle { x1, y1, x2, y2 } in input {
            out.push(Rectangle {
                x1: (x1 / round).round() * round,
                y1: (y1 / round).round() * round,
                x2: (x2 / round).round() * round,
                y2: (y2 / round).round() * round,
            })
        }
        return out;
    }

    fn svg_rectangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> svg::node::element::Rectangle {
        assert_eq!(self.colors.len(), self.color_weights.len());
        let dist = WeightedIndex::new(&self.color_weights).unwrap();
        let color = &self.colors[dist.sample(&mut self.rng)];
        svg::node::element::Rectangle::new()
            .set("x", x1)
            .set("y", y1)
            .set("width", x2 - x1)
            .set("height", y2 - y1)
            .set("fill", color.as_str())
    }

    fn svg_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> svg::node::element::Line {
        svg::node::element::Line::new()
            .set("x1", x1)
            .set("y1", y1)
            .set("x2", x2)
            .set("y2", y2)
            .set("style", format!("stroke:black;stroke-width:{}", self.line_width))
    }

    pub fn new_rng(&mut self) {
        if let Ok(o) = StdRng::from_rng(&mut thread_rng()) {
            self.rng = o
        }
    }
}
