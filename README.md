# Yew Spline
<p align="center">
<a href="https://crates.io/crates/yew_spline" target="_blank"><img alt="Crates.io" src="https://img.shields.io/crates/v/yew_spline?style=for-the-badge"></a>
<a href="https://crates.io/crates/yew_spline" target="_blank"><img alt="Crates.io" src="https://img.shields.io/crates/d/yew_spline?label=Downloads&style=for-the-badge"></a>
</p>
Yew Spline is a Spline runtime component for Yew based on Splines vanilla JS implementation, so you can display Spline scenes in Yew.   
   
Spline is a 3d editor that let's you make interactive 3d scenes to be viewed on the web. For more information visit [https://spline.design/](https://spline.design/).

## How to use
First you add `yew_spline` to your cargo.toml
```toml
[dependencies]
yew = "0.19.3^"
yew_spline = "..."
```
Then you use the <Spline /> component in your Html.

## Examples
__Use the splinecode URL__
```rust
use yew::prelude::*;
use yew_spline::spline::Spline;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Spline scene={"https://prod.spline.design/123/scene.splinecode"} />
    }
}
```

---

__Use a local splinecode file__   
If you download a splinecode file to use locally instead of loading it in
from the Spline servers, you have to link the directory it's in, in the `index.html` file. 

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Yew Spline</title>

    <link data-trunk rel="copy-dir" href="./src/assets">
</head>
<body>
    
</body>
</html>
```
In this example I have the standard src folder, but I have an assets folder with a spline folder nested inside.

```rust
use yew::prelude::*;
use yew_spline::spline::Spline;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Spline scene={"assets/spline/scene.splinecode"} />
    }
}
```

## License
Yew Spline is MIT licensed. See [license](LICENSE)
