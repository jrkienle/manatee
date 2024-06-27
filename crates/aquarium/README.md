# Aquarium

A cross-platform windowing library, built for Manatee and the Manatee Editor.

## Introduction

Like most things I've built, Aquarium was built out of pure spite, and this time my spite was
directed at Winit. Let me be very clear: **Winit is an amazing library and you should probably use
it instead of this.** With that being said, I wanted something that was a little more lightweight
than Winit in order to manage windowing for Manatee. I specifically wanted to be able to manage my
own event loop and build things with a more closure-forward architecture. Besides that, I also
wanted to see if I was even capable of building a cross-platform windowing library, and the answer
is... kinda? I'm gonna call this a win, score 1 for James!

## Features

As of right now, Aquarium does exactly two (2) things:

1. Opens windows {Only Windows and MacOS, I'll probably support more things soon(tm))
2. Lets you render things inside said windows with closures (I just think they're neat)

If you're looking for something to help you manage your event loop, access native system
functionality (other than opening some windows), or something that includes UI elements, this is
**definitely** not the package for you.

## Usage

```rust
use aquarium::{App, WindowParams};

pub fn main() {
    let app = App::new();
    app.run(|app_ctx| {
        app_ctx.new_window(WindowParams {
            title: "My Super Cool App Title",
            ..Default::default()
        })
    });
}

```

## License

This project is licensed under the MIT license. See [LICENSE](../../LICENSE) in the repo root for
more information.

## Acknowledgements

Aquarium's design was heavily influenced by the following work:

* [Winit](https://github.com/rust-windowing/winit)
* [Zed / GPUI](https://github.com/zed-industries/zed/)
