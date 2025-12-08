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
    let bmi_text_box = use_state(|| "24".to_string());



    let boy_parse_result = use_state(|| true);
    let kilo_parse_result = use_state(|| true);

    let boy_value = use_state(|| 174);
    let kilo_value = use_state(|| 75);

    let bmi_desc = use_state(|| "".to_string());

    let on_boy_text_change = {
        let boy_text_box = boy_text_box.clone();
        let boy_parse_result = boy_parse_result.clone();
        let boy_value = boy_value.clone();
        let kilo_parse_result = kilo_parse_result.clone();
        let kilo_value = kilo_value.clone();
        let bmi_text_box = bmi_text_box.clone();
        let bmi_desc = bmi_desc.clone();

        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            boy_text_box.set(input.value());
            let result = u32::from_str(&input.value());
            boy_parse_result.set(result.is_ok());

            if let Ok(boy_v) = result{
                boy_value.set(boy_v);
            }

            if *boy_parse_result && *kilo_parse_result{
                console::log_1(&"Boy change".into());
                let result_str = calculate_bmi(*boy_value, *kilo_value);
                bmi_text_box.set(result_str.0);
                bmi_desc.set(lookup_description(result_str.1));
            }else {
                console::log_1(&"Parse error".into());
            }


        })
    };

    let on_kilo_text_change = {
        let kilo_text_box = kilo_text_box.clone();
        let kilo_parse_result = kilo_parse_result.clone();
        let kilo_value = kilo_value.clone();
        let bmi_text_box = bmi_text_box.clone();
        let boy_parse_result = boy_parse_result.clone();
        let bmi_desc = bmi_desc.clone();

        let boy_value = boy_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            kilo_text_box.set(input.value());

            let result = u32::from_str(&input.value());
            kilo_parse_result.set(result.is_ok());

            if let Ok(kilo_v) = result{
                kilo_value.set(kilo_v);
            }

            if *boy_parse_result && *kilo_parse_result{
                console::log_1(&"Kilo change".into());
                let result_str = calculate_bmi(*boy_value, *kilo_value);
                bmi_text_box.set(result_str.0);
                bmi_desc.set(lookup_description(result_str.1));
            }else {
                console::log_1(&"Parse error".into());
            }


        })
    };

    fn calculate_bmi( boy:u32,kilo:u32) -> (String, f32){
        let boy_f = boy as f32 / 100.0;
        let bmi = kilo as f32 / (boy_f*boy_f) ;

        (format!("{:.2}", bmi), bmi)

    }

    fn lookup_description( val : f32) ->String{


        if val > 30.0{
            return "Obesity".to_string();
        }

        if val > 25.0{
            return "Overweight".to_string();
        }

        if val > 18.5{
            return "Normal".to_string();
        }

        "Underweight".to_string()
    }


    html! {
        <main style="font-family: sans-serif; max-width: 400px; margin: 20px auto;">
            <h1>{ "BMI Hesaplayıcı" }</h1>

            <label>
                { "Boy (cm) : " }
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
                { "Kilo (kg) : " }
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


            <br />

            <hr />

            <h3>{ "BMI" }</h3>
            <p>{ format!("{} ({})", *bmi_text_box, *bmi_desc) }</p>



        </main>
    }
}
