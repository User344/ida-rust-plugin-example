use autocxx::subclass::*;

mod ida;
use ida::bindgen;
use ida::bridge;
use ida::types::*;

mod plugin;
pub(crate) use plugin::*;

#[no_mangle]
pub extern "C" fn init() -> *const bindgen::plugmod_t {
    bridge::msg("Initializating ExamplePlugin\n");
    let plugin = ExamplePlugin::default_self_owned();
    let raw = std::rc::Rc::into_raw(plugin);
    unsafe { (*raw).as_ptr().as_ref().unwrap().as_ref() }
}

#[cfg(debug_assertions)]
pub const EXAMPLE_PLUGIN_FLAGS: i32 = PLUGIN_UNL | PLUGIN_MULTI;

#[cfg(not(debug_assertions))]
pub const EXAMPLE_PLUGIN_FLAGS: i32 = PLUGIN_MULTI;

#[no_mangle]
#[used]
pub static PLUGIN: plugin_t = plugin_t {
    version: IDP_INTERFACE_VERSION,
    flags: EXAMPLE_PLUGIN_FLAGS, // Plugin flags
    init: Some(init),            // Initialize function
    term: None,                  // Terminate function (None if not used)
    run: None,                   // Invoke function (None if not used)
    comment: std::ptr::null(),   // Long comment about the plugin
    help: std::ptr::null(),      // Multiline help about the plugin
    wanted_name: b"Test Rust plugin\0".as_ptr() as *const std::ffi::c_char, // Preferred short name
    wanted_hotkey: b"\0".as_ptr() as *const std::ffi::c_char, // Preferred hotkey
};
