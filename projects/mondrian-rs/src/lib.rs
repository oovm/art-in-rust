use rand::distributions::{Distribution};
use rand_distr::Beta;
use rand::Rng;

#[derive(Debug)]
struct Rectangle {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            x2: 1.618,
            y2: 1.0,
        }
    }
}

fn split(input: Vec<Rectangle>) -> Vec<Rectangle> {
    let mut out = vec![];
    let beta = Beta::new(10.0, 10.0).unwrap();
    let rng = &mut rand::thread_rng();
    for Rectangle {x1, y1, x2, y2} in input {
        let t = beta.sample(rng);
        let r = rng.gen_range(0.0, 1.0);
        if r < 0.3 * (x2 - x1) / (y2 - y1) {
            out.push(Rectangle {
                x1: x1,
                y1: y1,
                x2: x1 + (x2 - x1) * t,
                y2: y2,
            });
            out.push(Rectangle {
                x1: x1 + (x2 - x1) * t,
                y1: y1,
                x2: x2,
                y2: y2,
            });
        } else if 1.0 - r < 0.5 * (y2 - y1) / (x2 - x1) {
            out.push(Rectangle {
                x1: x1,
                y1: y1,
                x2: x2,
                y2: y1 + (y2 - y1) * t,
            });
            out.push(Rectangle {
                x1: x1,
                y1: y1 + (y2 - y1) * t,
                x2: x2,
                y2: y2,
            });
        }
        else {
            out.push(Rectangle {
                x1: x1,
                y1: y1,
                x2: x2,
                y2: y2,
            })
        }
    }
    return out
}



#[test]
fn test() {
    let mut v = vec![Rectangle::default()];
    for _ in 0..5 {
        v = split(v)
    }
    println!("{:#?}", v);
}