mod utils;

use std::f64::consts::PI;
use std::f64::INFINITY;
use std::fmt::Debug;
use std::sync::Mutex;

use chrono::prelude::*;
use chrono::{DateTime, Utc};

use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use utils::set_panic_hook;
use web_sys::{Document, Element, Event, HtmlInputElement, Storage, Window};

extern crate web_sys;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    fn setInterval(closure: &Closure<dyn FnMut()>, milliseconds: u32) -> f64;
    fn cancelInterval(token: f64);

    fn setTimeout(closure: &Closure<dyn FnMut()>, milliseconds: u32) -> f64;

    fn addEventListener(listener_type: &str, closure: &Closure<dyn FnMut(web_sys::Event)>) -> f64;
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EggParameters {
    radius: f64,
    specific_heat_capacity_white: f64,
    density_white: f64,
    thermal_conductivity_white: f64,
}

impl EggParameters {
    fn default() -> EggParameters {
        EggParameters {
            radius: 2.0,
            specific_heat_capacity_white: 3.7,
            density_white: 1.038,
            thermal_conductivity_white: 5.4e-3,
        }
    }

    fn from_mass(mass: f64) -> EggParameters {
        let mut parameters = EggParameters::default();
        parameters.radius =
            mass.powf(1.0 / 3.0) / ((4.0 * PI / 3.0).powf(1.0 / 3.0) * parameters.density_white);
        parameters
    }
}

#[derive(Debug, Clone, Copy)]
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
    let tau_0 = egg.specific_heat_capacity_white * egg.density_white * egg.radius.powi(2)
        / egg.thermal_conductivity_white;
    let temperature_from_formula = temperature_water
        + (temperature_egg_start - temperature_water)
            * (2.0 * egg.radius / (PI * radius))
            * (1..100)
                .map(|n| {
                    (f64::from((-1 as i32).pow(n - 1)) / f64::from(n))
                        * f64::sin(f64::from(n) * PI * radius / egg.radius)
                        * f64::exp(-(f64::from(n.pow(2)) * PI.powi(2) * time) / tau_0)
                })
                .sum::<f64>();
    if time <= 0.0 {
        temperature_egg_start
    } else if !(temperature_egg_start..temperature_water).contains(&temperature_from_formula) {
        temperature_egg_start
    } else {
        temperature_from_formula
    }
}

pub fn get_yolk_temperature(time: f64, parameters: &BoilSessionParameters) -> f64 {
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
    for _ in 1..100 {
        let temperature_difference =
            requested_yolk_temperature - get_yolk_temperature(guessed_time, parameters);
        if temperature_difference.abs() < 0.1 {
            return guessed_time;
        }
        guessed_time += temperature_difference * 5.0 * 60.0 / 70.0;
    }

    // TODO: Panick if debug mode

    return guessed_time;
}

pub fn get_document() -> Document {
    web_sys::window()
        .expect("No window")
        .document()
        .expect("No document")
}

pub fn query_selector(selector: &str) -> Element {
    get_document()
        .query_selector(selector)
        .expect(format!("Error when querying {selector}").as_str())
        .expect(format!("No element found matching query: {selector}").as_str())
}

pub fn get_window() -> Window {
    web_sys::window().expect("No window")
}

trait ElementTraitCustom {
    fn add_event_listener(&self, type_: &str, callback: Closure<dyn Fn(Event)>);
    fn get_value(&self) -> f64;
    fn set_value(&self, value: f64);
    fn add_class(&self, class: &str);
    fn remove_class(&self, class: &str);
}

impl ElementTraitCustom for Element {
    fn add_event_listener(&self, type_: &str, callback: Closure<dyn Fn(Event)>) {
        self.add_event_listener_with_callback(type_, &callback.as_ref().unchecked_ref())
            .expect("Adding event listener failed");
        callback.forget();
    }

    fn get_value(&self) -> f64 {
        self.to_owned()
            .dyn_into::<HtmlInputElement>()
            .expect("Couldn't cast Element to HtmlInputElement")
            .value_as_number()
    }

    fn set_value(&self, value: f64) {
        self.to_owned()
            .dyn_into::<HtmlInputElement>()
            .expect("Couldn't cast Element to HtmlInputElement")
            .set_value_as_number(value)
    }

    fn add_class(&self, class: &str) {
        self.set_class_name(format!("{} {}", self.class_name().as_str(), class).as_str());
    }

    fn remove_class(&self, class_to_remove: &str) {
        let old_class_name = self.class_name();
        let mut new_class_name = String::new();
        for class in old_class_name
            .split(" ")
            .filter(|&class| class != class_to_remove)
        {
            new_class_name.push_str(format!(" {class}").as_str());
        }
        self.set_class_name(&new_class_name);
    }
}

trait EventTraitCustom {
    fn target_element(&self) -> Element;
}

impl EventTraitCustom for Event {
    fn target_element(&self) -> Element {
        self.target()
            .expect("Event has no target")
            .dyn_ref::<Element>()
            .expect("Den sier dette")
            .to_owned()
    }
}

static boiling_start_mutex: Mutex<Option<DateTime<Utc>>> = Mutex::new(None);

static boiling_session_parameters_mutex: Mutex<BoilSessionParameters> =
    Mutex::new(BoilSessionParameters {
        temperature_water: 100.0,
        temperature_egg_start: 4.0,
        egg: EggParameters {
            radius: 2.0,
            specific_heat_capacity_white: 3.7,
            density_white: 1.038,
            thermal_conductivity_white: 5.4e-3,
        },
    });

static end_temperature_mutex: Mutex<f64> = Mutex::new(75.0);

pub fn set_mass_display(mass: f64) {
    query_selector("#mass-display").set_inner_html(format!("{:.1} g", mass).as_str());
}

pub fn set_boiling_temperature_display(temperature: f64) {
    // query_selector("#boiling-temperature-display")
    //     .set_inner_html(format!("{:.1} °C", temperature).as_str());
}

pub fn set_start_temperature_display(temperature: f64) {
    query_selector("#start-temperature-display")
        .set_inner_html(format!("{:.1} °C", temperature).as_str());
}

pub fn set_end_temperature_display(temperature: f64) {
    query_selector("#end-temperature-display")
        .set_inner_html(format!("{:.1} °C", temperature).as_str());
    query_selector("#end-temperature-display-2")
        .set_inner_html(format!("{:.1} °C", temperature).as_str());
    query_selector("#end-temperature-display-3")
        .set_inner_html(format!("{:.1} °C", temperature).as_str());
}

pub fn set_end_temperature_boiling_time_display(time: f64) {
    query_selector(format!("#end-temperature-boiling-time-display").as_str())
        .set_inner_html(format!("{:.0} min {:.0} s", time as i32 / 60, time as i32 % 60).as_str());
}

pub fn set_time_since_start_display(time: f64) {
    query_selector("#time-since-start-display")
        .set_inner_html(format!("{:.0} min {:.0} s", time as i32 / 60, time as i32 % 60).as_str());
}

pub fn set_yolk_temperature_display(temperature: f64) {
    query_selector("#yolk-temperature-display")
        .set_inner_html(format!("{:.1} °C", temperature).as_str());
}

pub fn set_boiling_time_x_degrees_display(x: i32, time: f64) {
    query_selector(format!("#boiling-time-{x}-degrees-display").as_str())
        .set_inner_html(format!("{:.0} min {:.0} s", time as i32 / 60, time as i32 % 60).as_str());
}

pub fn set_time_till_end_temperature_display(time: f64) {
    query_selector(format!("#time-till-end-temperature-display").as_str())
        .set_inner_html(format!("{:.0} min {:.0} s", time as i32 / 60, time as i32 % 60).as_str());
}

pub fn update_outputs() {
    let parameters = *boiling_session_parameters_mutex.lock().unwrap();
    let end_temperature_boiling_time =
        get_boiling_time(*end_temperature_mutex.lock().unwrap(), &parameters);
    if let Some(boiling_start) = *boiling_start_mutex.lock().unwrap() {
        let time = Utc::now()
            .signed_duration_since(boiling_start)
            .num_milliseconds() as f64
            * 0.001;
        set_time_since_start_display(time);
        set_yolk_temperature_display(get_yolk_temperature(time, &parameters));
        set_time_till_end_temperature_display(end_temperature_boiling_time - time);
    }
    set_boiling_time_x_degrees_display(70, get_boiling_time(70.0, &parameters));
    set_boiling_time_x_degrees_display(75, get_boiling_time(75.0, &parameters));
    set_boiling_time_x_degrees_display(80, get_boiling_time(80.0, &parameters));
    set_boiling_time_x_degrees_display(85, get_boiling_time(85.0, &parameters));
    set_end_temperature_boiling_time_display(end_temperature_boiling_time);
}

pub fn get_local_storage() -> Storage {
    get_window()
        .local_storage()
        .expect("Failed to get localStorage of window")
        .expect("Window has no localStorage")
}

pub fn set_local_storage<T>(key: &str, value: T)
where
    T: Serialize + DeserializeOwned + Debug,
{
    let storage = get_local_storage();
    let value = serde_json::to_string(&value)
        .expect(format!("Failed to deserialize value {value:?}").as_str());
    storage
        .set_item(key, &value)
        .expect(format!("Failed to store value {value:?} in localStorage.{key}").as_str());
}

pub fn local_storage<T>(key: &str) -> Option<T>
where
    T: Serialize + DeserializeOwned,
{
    let storage = get_local_storage();
    if let Ok(Some(raw)) = storage.get_item(key) {
        if let Ok(value) = serde_json::from_str::<Option<T>>(raw.as_str()) {
            return value;
        }
    }
    None
}

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();

    if let Some(mass) = local_storage("mass") {
        query_selector("#mass-input").set_value(mass);
    }
    if let Some(boiling_temperature) = local_storage("boiling-temperature") {
        query_selector("#boiling-temperature-input").set_value(boiling_temperature);
    }
    if let Some(start_temperature) = local_storage("start-temperature") {
        query_selector("#start-temperature-input").set_value(start_temperature);
    }
    if let Some(end_temperature) = local_storage("end-temperature") {
        query_selector("#end-temperature-input").set_value(end_temperature);
    }
    if let Some(boiling_start) = local_storage("boiling-start") {
        *boiling_start_mutex.lock().unwrap() = boiling_start;
        if boiling_start.is_some() {
            let button = query_selector("#start-button");
            let yolk_temperature_display_wrapper =
                query_selector("#yolk-temperature-display-wrapper");
            button.set_inner_html("Stopp koking");
            button.remove_class("btn-success");
            button.add_class("btn-danger");
            yolk_temperature_display_wrapper.remove_class("invisible");
            yolk_temperature_display_wrapper.add_class("visible");
        }
    }

    let mass = query_selector("#mass-input").get_value();
    (*boiling_session_parameters_mutex.lock().unwrap()).egg = EggParameters::from_mass(mass);
    set_mass_display(mass);
    query_selector("#mass-input").add_event_listener(
        "input",
        Closure::<dyn Fn(_)>::new(|event: Event| {
            let mass = event.target_element().get_value();
            (*boiling_session_parameters_mutex.lock().unwrap()).egg =
                EggParameters::from_mass(mass);
            set_mass_display(mass);
            update_outputs();
            set_local_storage("mass", mass);
        }),
    );

    let boiling_temperature = query_selector("#boiling-temperature-input").get_value();
    (*boiling_session_parameters_mutex.lock().unwrap()).temperature_water = boiling_temperature;
    set_boiling_temperature_display(boiling_temperature);
    query_selector("#boiling-temperature-input").add_event_listener(
        "input",
        Closure::<dyn Fn(_)>::new(|event: Event| {
            let boiling_temperature = event.target_element().get_value();
            (*boiling_session_parameters_mutex.lock().unwrap()).temperature_water =
                boiling_temperature;
            set_boiling_temperature_display(boiling_temperature);
            update_outputs();
            set_local_storage("boiling-temperature", boiling_temperature);
        }),
    );

    let start_temperature = query_selector("#start-temperature-input").get_value();
    (*boiling_session_parameters_mutex.lock().unwrap()).temperature_egg_start = start_temperature;
    set_start_temperature_display(start_temperature);
    query_selector("#start-temperature-input").add_event_listener(
        "input",
        Closure::<dyn Fn(_)>::new(|event: Event| {
            let start_temperature = event.target_element().get_value();
            (*boiling_session_parameters_mutex.lock().unwrap()).temperature_egg_start =
                start_temperature;
            set_start_temperature_display(start_temperature);
            update_outputs();
            set_local_storage("start-temperature", start_temperature);
        }),
    );

    let end_temperature = query_selector("#end-temperature-input").get_value();
    (*end_temperature_mutex.lock().unwrap()) = end_temperature;
    set_end_temperature_display(end_temperature);
    query_selector("#end-temperature-input").add_event_listener(
        "input",
        Closure::<dyn Fn(_)>::new(|event: Event| {
            let end_temperature = event.target_element().get_value();
            (*end_temperature_mutex.lock().unwrap()) = end_temperature;
            set_end_temperature_display(end_temperature);
            update_outputs();
            set_local_storage("end-temperature", end_temperature);
        }),
    );

    query_selector("#start-button").add_event_listener(
        "click",
        Closure::<dyn Fn(_)>::new(|event: Event| {
            {
                let mut boiling_start = boiling_start_mutex.lock().unwrap();
                let yolk_temperature_display_wrapper =
                    query_selector("#yolk-temperature-display-wrapper");
                if (*boiling_start).is_none() {
                    *boiling_start = Some(Utc::now());
                    let button = event.target_element();
                    button.set_inner_html("Stopp koking");
                    button.remove_class("btn-success");
                    button.add_class("btn-danger");
                    yolk_temperature_display_wrapper.remove_class("invisible");
                    yolk_temperature_display_wrapper.add_class("visible");
                } else {
                    *boiling_start = None;
                    let button = event.target_element();
                    button.set_inner_html("Start koking");
                    button.remove_class("btn-danger");
                    button.add_class("btn-success");
                    yolk_temperature_display_wrapper.remove_class("visible");
                    yolk_temperature_display_wrapper.add_class("invisible");
                }
                set_local_storage("boiling-start", *boiling_start);
            }
            update_outputs();
        }),
    );

    let boiling_interval_closure = Closure::<dyn FnMut()>::new(|| {
        update_outputs();
    });
    setInterval(&boiling_interval_closure, 100);

    boiling_interval_closure.forget();
}
