# Neptune
*A brand new free, open-source minimal and compact game engine built in rust.*

## Design Goals
We plan to make Neptune a small and minimal engine, making it easy to learn and most importantly modify to your needs, and not having features that aren't often used.

## *Warning*
**Neptune is still in very early stages, and won't be an actual usable engine for a long time, and may not ever become a complete one.**

## Getting Started
Currently the engine doesn't have much to do other than creating a window, but I will be showing examples of how you can get started.

```rust
use neptune_eng::graphics::window::*;

fn main() {
    let mut window = Window::new(800, 600, "Window Title");
    window.init_gl();

    while !window.should_close() {
        window.update();
    }
}
```
*The example above shows you how to create a window using neptune.*

## Contributions
If you want to contribute just open a pull request, if you have any issues make sure to also post them, I will always be accepting pull requests and doing my best to keep this updated and working.
