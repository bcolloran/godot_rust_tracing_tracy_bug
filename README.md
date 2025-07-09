Adding `use tracing_tracy::TracyLayer;` in you gdextension Rust code causes an error in Godot when the project is loaded or when hot reloading occurs. The error message is:

```
ERROR: core/extension/gdextension_library_loader.cpp:233 - GDExtension initialization function 'gdext_rust_init' returned an error.
```

To reproduce:
- Uncomment the line `use tracing_tracy::TracyLayer;` in `rust/src/lib.rs`
- cargo build
- the error will appear in the Godot editor console
