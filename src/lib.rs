mod utils;

use std::array;
use std::f64::INFINITY;
use std::f64::consts::PI;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::sync::Arc;

// extern crate chrono;

// use chrono::Local;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use utils::set_panic_hook;
use web_sys::{Window, HtmlElement, HtmlInputElement};

extern crate web_sys;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // #[wasm_bindgen(js_namespace = console)]
    // fn log(s: &str);

    #[wasm_bindgen(js_namespace = document)]
    fn querySelector(s: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

struct Input {
    html_input_selector: String,
    html_display_selector: String,
}

pub struct EggParameters {
    radius: f64,
    specific_heat_capacity_white: f64,
    density_white: f64,
    thermal_conductivity_white: f64,
}

impl EggParameters {
    fn default() -> EggParameters {
        EggParameters {
            radius: 1.0,
            specific_heat_capacity_white: 3.7,
            density_white: 1.038,
            thermal_conductivity_white: 5.4e-3,
        }
    }

    fn from_mass(mass: f64) -> EggParameters {
        let mut parameters = EggParameters::default();
        parameters.radius = mass.powf(1.0 / 3.0) / ((4.0 * PI / 3.0).powf(1.0 / 3.0) * parameters.density_white);
        parameters
    }
}

pub struct BoilSessionParameters {
    temperature_water: f64,
    temperature_egg_start: f64,
    egg: EggParameters,
}

pub fn get_egg_temperature_at_radius(
    radius: f64,
    time: f64,
    parameters: &BoilSessionParameters,
) -> f64 {
    let temperature_water = parameters.temperature_water;
    let temperature_egg_start = parameters.temperature_egg_start;
    let egg = &parameters.egg;
    let tau_0 = egg.specific_heat_capacity_white
        * egg.density_white
        * egg.radius.powi(2)
        / egg.thermal_conductivity_white;
    return temperature_water
        + (temperature_egg_start - temperature_water)
            * (2.0 * egg.radius / (PI * radius))
            * (1..100)
                .map(|n| {
                    (f64::from((-1 as i32).pow(n - 1)) / f64::from(n))
                        * f64::sin(f64::from(n) * PI * radius / egg.radius)
                        * f64::exp(-(f64::from(n.pow(2)) * PI.powi(2) * time) / tau_0)
                })
                .sum::<f64>();
}

pub fn get_yolk_temperature(
    time: f64,
    parameters: &BoilSessionParameters,
) -> f64 {
    get_egg_temperature_at_radius(0.69 * parameters.egg.radius, time, parameters)
}

pub fn get_boiling_time(
    requested_yolk_temperature: f64,
    parameters: &BoilSessionParameters,
) -> f64 {
    let mut guessed_time = 0.0;
    if requested_yolk_temperature < parameters.temperature_egg_start {
        0.0;
    }
    if requested_yolk_temperature > parameters.temperature_water {
        return INFINITY;
    }
    loop {
        let temperature_difference = requested_yolk_temperature - get_yolk_temperature(guessed_time, parameters);
        if temperature_difference.abs() < 0.1 {
            return guessed_time;
        }
        guessed_time += temperature_difference * f64::from(30 * 60 / 100);
    }
}

pub fn read_html_input_as_number(body: &HtmlElement, selector: &str) -> f64 {
    let input = body.query_selector(selector).unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    input.value_as_number()
}

pub fn set_inner_html(body: &HtmlElement, selector: &str, value: &str) {
    let element = body.query_selector(selector).unwrap().unwrap();
    element.set_inner_html(value)
}

#[wasm_bindgen]
pub struct BoilingSession {
    boiling_start: f64,
}

#[wasm_bindgen]
impl BoilingSession {
    pub fn new() -> BoilingSession {
        let window: Window = web_sys::window().expect("No window");
        let document = window.document().expect("No document");
        let body = document.body().expect("No body");
        let performance_object = window.performance().expect("No performance object");
        BoilingSession {
            boiling_start: performance_object.now(),
        }

        // let input_callback = Closure::<dyn FnMut()>::new(move || session.update_display());
        // body.query_selector("#mass-display").expect("No #thebutton").unwrap().add_event_listener_with_callback("input", &input_callback.as_ref().unchecked_ref());
        
        // input_callback.forget();
        // session
    }

    pub fn update_display(&mut self) {
        let window: Window = web_sys::window().expect("No window");
        let document = window.document().expect("No document");
        let body = document.body().expect("No body");
        let performance_object = window.performance().expect("No performance object");
        let time_ms = performance_object.now() - self.boiling_start;
        let egg_mass = read_html_input_as_number(&body, "#mass-input");
        let parameters = BoilSessionParameters {
            temperature_water: read_html_input_as_number(&body, "#boiling-temperature-input"),
            temperature_egg_start: read_html_input_as_number(&body, "#start-temperature-input"),
            egg: EggParameters::from_mass(egg_mass),
        };
        let yolk_temperature = get_yolk_temperature(0.001 * time_ms, &parameters);
        // let yolk_temperature = get_yolk_temperature(300.0, &parameters);
        // log!("Yolk temperature: {:?}", yolk_temperature);

        let boiling_time_75_degrees = get_boiling_time(75.0, &parameters);
        // chrono::Duration::from_std(Duration(boiling_time_75_degrees))
        // Duration::from_secs(boiling_time_75_degrees).subsec_millis()

        


        set_inner_html(&body, "#mass-display", format!("{:.1} g", egg_mass).as_str());
        set_inner_html(&body, "#boiling-temperature-display", format!("{:.1} °C", parameters.temperature_water).as_str());
        set_inner_html(&body, "#start-temperature-display", format!("{:.1} °C", parameters.temperature_egg_start).as_str());
        set_inner_html(&body, "#yolk-temperature-display", format!("{:.1} °C", yolk_temperature).as_str());
        set_inner_html(&body, "#boiling-time-75-degrees-display", format!("{:.0} min {:.0} s", boiling_time_75_degrees as i32 / 60, boiling_time_75_degrees as i32 % 60 ).as_str());
    }
}


#[wasm_bindgen]
pub fn init() {
    // alert("Hello, seba2!");

    let inputs = vec![
        Input {
            html_input_selector: "#mass-input".to_string(),
            html_display_selector: "#mass-display".to_string(),
        },
        Input {
            html_input_selector: "#boiling-temperature-input".to_string(),
            html_display_selector: "#boiling-temperature-display".to_string(),
        },
        Input {
            html_input_selector: "#start-temperature-input".to_string(),
            html_display_selector: "#start-temperature-display".to_string(),
        },
    ];
        
    log!("Hei");
    
    let window: Window = web_sys::window().expect("No window");
    // let performance_object = window.performance().expect("No performance");

    // pub fn input_changed() {}
    // window.set_interval_with_callback_and_timeout_and_arguments_0(&timer_callback_object, 1000);
}
