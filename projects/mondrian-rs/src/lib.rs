mod algorithm;

pub use algorithm::{Rectangle,rectangle_svg,split,rectangle_round};
use svg::Document;


pub struct mondrian {

}

impl mondrian {
    pub fn generate() {

    }
}


pub fn mondrian(nest: usize, round: f32, line: f32) -> Document {
    let mut v = vec![Rectangle::default()];
    for _ in 0..nest {
        v = split(v)
    }
    v = rectangle_round(v, round);
    rectangle_svg(v,line)
}


#[test]
fn test() {
    let s = mondrian(10, 0.10,0.5);
    svg::save("image.svg", &s).unwrap();
}
