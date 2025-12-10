use yew::prelude::*;
use yew::Renderer;

#[function_component]
fn App() -> Html {
    let weight = use_state(|| 70);
    let height = use_state(|| 170);

    let on_weight_change = {
        let weight = weight.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = input {
                if let Ok(value) = input.value().parse::<i32>() {
                    weight.set(value);
                }
            }
        })
    };

    let on_height_change = {
        let height = height.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = input {
                if let Ok(value) = input.value().parse::<i32>() {
                    height.set(value);
                }
            }
        })
    };

    // Calculate BMI: weight (kg) / (height (m))^2
    let height_meters = *height as f64 / 100.0;
    let bmi = (*weight as f64) / (height_meters * height_meters);
    let bmi_rounded = (bmi * 10.0).round() / 10.0;

    // Determine BMI category
    let (category, category_class) = if bmi < 18.5 {
        ("Underweight", "bmi-underweight")
    } else if bmi < 25.0 {
        ("Normal", "bmi-normal")
    } else if bmi < 30.0 {
        ("Overweight", "bmi-overweight")
    } else {
        ("Obese", "bmi-obese")
    };

    html! {
        <div class="container">
            <h1 class="title">{ "BMI Calculator" }</h1>
            
            <div class="card">
                <h2 class="card-title">{ "Weight" }</h2>
                <div class="slider-container">
                    <div class="slider-value-display">
                        <span class="slider-label">{ "Weight: " }</span>
                        <span class="slider-number">{ *weight }</span>
                        <span class="slider-unit">{ " kg" }</span>
                    </div>
                    <input
                        type="range"
                        min="40"
                        max="150"
                        value={(*weight).to_string()}
                        oninput={on_weight_change}
                        class="slider"
                    />
                    <div class="slider-labels">
                        <span>{ "40 kg" }</span>
                        <span>{ "95 kg" }</span>
                        <span>{ "150 kg" }</span>
                    </div>
                </div>
            </div>

            <div class="card">
                <h2 class="card-title">{ "Height" }</h2>
                <div class="slider-container">
                    <div class="slider-value-display">
                        <span class="slider-label">{ "Height: " }</span>
                        <span class="slider-number">{ *height }</span>
                        <span class="slider-unit">{ " cm" }</span>
                    </div>
                    <input
                        type="range"
                        min="120"
                        max="220"
                        value={(*height).to_string()}
                        oninput={on_height_change}
                        class="slider"
                    />
                    <div class="slider-labels">
                        <span>{ "120 cm" }</span>
                        <span>{ "170 cm" }</span>
                        <span>{ "220 cm" }</span>
                    </div>
                </div>
            </div>

            <div class="card bmi-result-card">
                <h2 class="card-title">{ "Your BMI" }</h2>
                <div class="bmi-display">
                    <span class="bmi-value">{ bmi_rounded }</span>
                    <span class={format!("bmi-category {}", category_class)}>{ category }</span>
                </div>
            </div>

            <div class="card">
                <h2 class="card-title">{ "BMI Categories" }</h2>
                <div class="bmi-categories">
                    <div class="category-item">
                        <span class="category-indicator bmi-underweight"></span>
                        <span class="category-text">{ "Underweight: < 18.5" }</span>
                    </div>
                    <div class="category-item">
                        <span class="category-indicator bmi-normal"></span>
                        <span class="category-text">{ "Normal: 18.5 - 24.9" }</span>
                    </div>
                    <div class="category-item">
                        <span class="category-indicator bmi-overweight"></span>
                        <span class="category-text">{ "Overweight: 25.0 - 29.9" }</span>
                    </div>
                    <div class="category-item">
                        <span class="category-indicator bmi-obese"></span>
                        <span class="category-text">{ "Obese: ≥ 30.0" }</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}

