pub mod spline;
pub mod spline_sys;

use yew::prelude::*;

use crate::spline::Spline;

#[function_component(App)]
pub fn app() -> Html {
    

    html! {
        <>
            <h1>{"Yew Spline Design Test"}</h1>
            <Spline url={"assets/spline/scene.splinecode"} />
        </>
    }
}
