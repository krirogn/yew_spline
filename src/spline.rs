use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::spline_sys::Application;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub scene: String,
}

#[function_component(Spline)]
pub fn spline(props: &Props) -> Html {
    // 
    let canvas: NodeRef = NodeRef::default();

    // Clone to move in effect hook
    let canvas_clone = canvas.clone();
    let props_clone = props.clone();
    use_effect(move || {
        // After render
        if let Some(canvas_element) = canvas_clone.cast::<HtmlCanvasElement>() {
            let spline = Application::new(&canvas_element);
            spline.load(props_clone.scene);
        }

        // Cleanup
        || ()
    });

    html! {
        <canvas ref={canvas}></canvas>
    }
}