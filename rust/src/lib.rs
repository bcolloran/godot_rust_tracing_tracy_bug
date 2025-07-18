use godot::prelude::*;

#[cfg(feature = "profiling")]
// Single global handle; will be initialised exactly once.
static TRACY_CLIENT: std::sync::OnceLock<tracing_tracy::client::Client> =
    std::sync::OnceLock::new();

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(_level: InitLevel) {
        // can twiddle this string to make sure the library is hot reloaded
        godot_print!("MyExtension::on_level_init - [edit 3]");
        #[cfg(feature = "profiling")]
        {
            use std::sync::Once;
            use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
            use tracing_tracy::{TracyLayer, client::Client};

            if _level == InitLevel::Scene && !godot::classes::Engine::singleton().is_editor_hint() {
                godot_print!("MyExtension::on_level_init, with tracing enabled");
                // Make sure we only run the init block once per library load.
                static START: Once = Once::new();
                START.call_once(|| {
                    // 1. Start Tracy manually (manual‑lifetime feature enabled).
                    let client = Client::start();
                    let _ = TRACY_CLIENT.set(client);

                    // 2. Install the Tracy layer for all `tracing` spans.
                    let _ = tracing_subscriber::registry()
                        .with(TracyLayer::default())
                        .try_init(); // avoids panics if already set
                });
            }
        }
    }

    fn on_level_deinit(_level: InitLevel) {
        godot_print!("MyExtension::on_level_deinit");
        #[cfg(feature = "profiling")]
        {
            if _level == InitLevel::Scene && !godot::classes::Engine::singleton().is_editor_hint() {
                godot_print!("MyExtension::on_level_deinit, with tracing enabled");
                // Explicitly shut Tracy down; required with `manual-lifetime`.
                unsafe {
                    tracing_tracy::client::sys::___tracy_shutdown_profiler();
                }
                // TRACY_CLIENT stays filled, but the library is about to be unloaded,
                // so its memory will disappear immediately afterwards.
            }
        }
    }
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
        godot_print!("Tester::init");
        Self {
            base,
            process_tick: 0,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        // can twiddle this string to make sure the library is hot reloaded,
        // and that updated custom node methods are called correctly
        godot_print!(
            "Tester::process, process_tick [edit 3]: {}",
            self.process_tick
        );
        self.process_tick += 1;
        let a = fib(10);
        let b = next_prime(10000);
        godot_print!("    fib: {}, next_prime: {}", a, b);
    }
}

#[tracing::instrument]
fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

#[tracing::instrument]
fn next_prime(n: u32) -> u32 {
    let mut candidate = n + 1;
    while !is_prime(candidate) {
        candidate += 1;
    }
    // can twiddle this string to make sure the library is hot reloaded,
    // and that updated rust fns called correctly
    godot_print!("next_prime({}) = {}; [edit 3]", n, candidate);
    candidate
}

#[tracing::instrument]
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
