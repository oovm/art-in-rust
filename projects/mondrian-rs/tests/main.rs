use mondrian::{save, Mondrian};

#[test]
fn test() {
    let mut s = Mondrian::default();
    save("iter-5.svg", &s.generate(5)).unwrap();

    s.new_rng(); // Otherwise it will generate exactly the same graphics
    s.line_width = 0.8;
    save("iter-10.svg", &s.generate(10)).unwrap();
}
