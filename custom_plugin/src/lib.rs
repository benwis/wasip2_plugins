use plugin_api::bindings::export;
use plugin_api::Plugin;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn greeting(name: String) -> String {
        println!("STDIO WORKS!");
        format!("Greetings {name}! I'm a WASI plugin!")
    }
}
export!(MyPlugin with_types_in plugin_api::bindings);
