#![recursion_limit = "1024"]
use mondrian::{
    rand::{prelude::StdRng, SeedableRng},
    thread_rng, Mondrian,
};
use std::str::FromStr;
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub fn header_view() -> Html {
    let title = "Piet Mondrian Artworks";
    html! {
    <header>
        <h1 color="#009688">{title}</h1>
        <a href="https://github.com/GalAster/mondrian-rs">{"Fork me!"}</a>
    </header>
    }
}

pub enum Event {
    Iteration(ChangeData),
    Width(ChangeData),
    LineWidth(ChangeData),
    GridRound(ChangeData),
}

pub struct Model {
    link: ComponentLink<Self>,
    iterations: u32,
    line_width: f32,
    grid_round: f32,
    width: f32,
    rng: StdRng,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, iterations: 5, line_width: 1.0, grid_round: 0.05, width: 1.6, rng: new_rng() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Width(s) => match s {
                ChangeData::Value(s) => match f32::from_str(s.as_str()) {
                    Ok(o) => self.width = o,
                    _ => (),
                },
                _ => {}
            },
            Event::LineWidth(s) => match s {
                ChangeData::Value(s) => match f32::from_str(s.as_str()) {
                    Ok(o) => self.line_width = o,
                    _ => (),
                },
                _ => {}
            },
            Event::GridRound(s) => match s {
                ChangeData::Value(s) => match f32::from_str(s.as_str()) {
                    Ok(o) => self.grid_round = o,
                    _ => (),
                },
                _ => {}
            },
            Event::Iteration(s) => match s {
                ChangeData::Value(s) => match u32::from_str(s.as_str()) {
                    Ok(o) => {
                        self.iterations = o;
                        self.rng = new_rng()
                    }
                    _ => (),
                },
                _ => {}
            },
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                {header_view()}
                <main>

                <div class="control">
                <label>{"Iterations:"}</label>
                <div class="range">
                <input
                type="range"
                min="1"
                max="12"
                step="1"
                class="slider"
                value=self.iterations
                onchange=self.link.callback(|input: ChangeData| Event::Iteration(input))
                />
                <span class="tooltip">{self.iterations}</span>
                </div>
                </div>

                <div class="control">
                <label>{"Width:"}</label>
                <div class="range">
                <input
                type="range"
                min="1"
                max="4"
                step="0.01"
                class="slider"
                value=self.width
                onchange=self.link.callback(|input: ChangeData| Event::Width(input))
                />
                <span class="tooltip">{self.width}</span>
                </div>
                </div>

                <div class="control">
                <label>{"Line Width:"}</label>
                <div class="range">
                <input
                type="range"
                min="0"
                max="2"
                step="0.01"
                class="slider"
                value=self.line_width
                onchange=self.link.callback(|input: ChangeData| Event::LineWidth(input))
                />
                <span class="tooltip">{self.line_width}</span>
                </div>
                </div>

                <div class="control">
                <label>{"Grid Round:"}</label>
                <div class="range">
                <input
                type="range"
                min="0.01"
                max="0.20"
                step="0.01"
                class="slider"
                value=self.grid_round
                onchange=self.link.callback(|input: ChangeData| Event::GridRound(input))
                />
                <span class="tooltip">{self.grid_round}</span>
                </div>
                </div>

                {self.generate()}
                </main>
            </>
        }
    }
}

impl Model {
    fn generate(&self) -> Html {
        let mut config = Mondrian {
            line_width: self.line_width,
            rng: self.rng.clone(),
            grid_round: self.grid_round,
            width: self.width,
            ..Mondrian::default()
        };
        let t = yew::utils::document().create_element("div").unwrap();
        t.set_id("playground");
        t.set_inner_html(&format!("{}", config.generate(self.iterations)));
        Html::VRef(t.into())
    }
}

fn new_rng() -> StdRng {
    StdRng::from_rng(&mut thread_rng()).unwrap()
}

fn main() {
    yew::start_app::<Model>();
}
