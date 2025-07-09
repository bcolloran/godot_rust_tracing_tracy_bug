use godot::prelude::*;
// use std::sync::Once;

// importing `tracing_subscriber`` is ok
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/*
but importing `tracing_tracy` causes an error:
```
ERROR: core/extension/gdextension_library_loader.cpp:233 - GDExtension initialization function 'gdext_rust_init' returned an error.
```

To reproduce:
- Uncomment the line below to see the error
- cargo build
- the error will appear in the Godot editor console

This happen when hot reloading and when closing the project and reopening it.
*/
// use tracing_tracy::TracyLayer;

struct MyExtension;
#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    // Code recommended by ChatGPT to initialize the extension, but haven't tested this because the `tracing_tracy` import causes an error.

    // Start as early as possible
    // fn min_level() -> InitLevel {
    //     InitLevel::Core // can also be InitLevel::Scene if you prefer
    // }

    // fn on_level_init(level: InitLevel) {
    //     if level == InitLevel::Core {
    //         // Prevent re‑registering on hot‑reload
    //         static START: Once = Once::new();
    //         START.call_once(|| {
    //             let subscriber = tracing_subscriber::registry().with(TracyLayer::default());

    //             // try_init() ignores "subscriber already set" errors gracefully
    //             let _ = subscriber.try_init();
    //         });
    //     }
    // }
}

#[derive(GodotClass)]
#[class(base=Node)]
struct Tester {
    base: Base<Node>,
    #[var]
    process_tick: u32,
}

#[godot_api]
impl INode for Tester {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            process_tick: 0,
        }
    }

    fn process(&mut self, _delta: f64) {
        godot_print!("Tester::process, process_tick: {}", self.process_tick);
        self.process_tick += 1;
        let a = fib(10);
        let b = next_prime(100);
        godot_print!("    fib: {}, next_prime: {}", a, b);
    }
}

// Some testing functions to trace
// Uncomment the `#[tracing::instrument]` attribute to enable tracing for these functions (NOT WORKING YET)

// #[tracing::instrument]
fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

// #[tracing::instrument]
fn next_prime(n: u32) -> u32 {
    let mut candidate = n + 1;
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}

// #[tracing::instrument]
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
