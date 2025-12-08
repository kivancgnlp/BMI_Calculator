use std::fmt::format;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::console;

// Entry point called from JS/WASM glue.
#[wasm_bindgen(start)]
pub fn run_app() {
    // mount the Yew app into <div id="root">
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    // State for the two textboxes
    let boy_text_box = use_state(|| "174".to_string());
    let kilo_text_box = use_state(|| "75".to_string());
    let bmi_text_box = use_state(|| String::new());



    let boy_parse_result = use_state(|| true);
    let kilo_parse_result = use_state(|| true);

    let boy_value = use_state(|| 174);
    let kilo_value = use_state(|| 75);

    let on_boy_text_change = {
        let boy_text_box = boy_text_box.clone();
        let boy_parse_result = boy_parse_result.clone();
        let boy_value = boy_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            boy_text_box.set(input.value());
            let result = u32::from_str(&input.value());
            boy_parse_result.set(result.is_ok());

            if let Ok(boy_v) = result{
                boy_value.set(boy_v);
            }


        })
    };

    let on_kilo_text_change = {
        let kilo_text_box = kilo_text_box.clone();
        let kilo_parse_result = kilo_parse_result.clone();
        let kilo_value = kilo_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            kilo_text_box.set(input.value());

            let result = u32::from_str(&input.value());
            kilo_parse_result.set(result.is_ok());

            if let Ok(kilo_v) = result{
                kilo_value.set(kilo_v);
            }
        })
    };

    fn calculate_bmi( boy:u32,kilo:u32){

    }

    let calculate = {

       let bmi_text_box = bmi_text_box.clone();

        let boy_parse_result = boy_parse_result.clone();
        let kilo_parse_result = kilo_parse_result.clone();

        let boy_value = boy_value.clone();
        let kilo_value = kilo_value.clone();
        Callback::from(move |_| {

           if *boy_parse_result && *kilo_parse_result{
               console::log_1(&"Calc".into());
               let boy_f = *boy_value as f32 / 100.0;
               let bmi = *kilo_value as f32 / (boy_f*boy_f) ;
               console::log_1(&format!("BMI {}", bmi).into());
               bmi_text_box.set(format!("{:.2} kg/m2", bmi).to_string());
           }else {
                console::log_1(&"Parse error".into());
            }


        })
    };



    html! {
        <main style="font-family: sans-serif; max-width: 400px; margin: 20px auto;">
            <h1>{ "BMI Hesaplayıcı" }</h1>

            <label>
                { "Boy : " }
                <input
                    type="number"
                    value={(*boy_text_box).clone()}
                    oninput={on_boy_text_change}
                    style={
                        if *boy_parse_result {
                            "".to_string()
                        } else {
                            "border: 1px solid red;".to_string()
                        }
                    }

                />
            </label>
            <br /><br />

            <label>
                { "Kilo : " }
                <input
                    type="number"
                    value={(*kilo_text_box).clone()}
                    oninput={on_kilo_text_change}
                    style={
                        if *kilo_parse_result {
                            "".to_string()
                        } else {
                            "border: 1px solid red;".to_string()
                        }
                    }
                />
            </label>
            <br /><br />

            <label>
                { "BMI : " }
                <input
                    type="text"
                    value={(*bmi_text_box).clone()}
                    readonly=true

                />
            </label>

            <br /><br />
            <button onclick={calculate}>{ "Hesapla" }</button>

            <br />

            <hr />

            <h3>{ "Debug view" }</h3>
            <p>{ format!("Boy: {}", *boy_text_box) }</p>
            <p>{ format!("Kilo: {}", *kilo_text_box) }</p>


        </main>
    }
}
