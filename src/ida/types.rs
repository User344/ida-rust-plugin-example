use crate::ida::bindgen::plugmod_t;
use cxx::{type_id, ExternType};

#[allow(non_camel_case_types)]
pub type ea_t = u64;

// Hardcoded for IDA 7.5
pub const IDP_INTERFACE_VERSION: i32 = 700;

/// Plugin changes the database
#[allow(unused)]
pub const PLUGIN_MOD: i32 = 0x0001;

/// IDA should redraw after plugin call
#[allow(unused)]
pub const PLUGIN_DRAW: i32 = 0x0002;

/// Plugin may be applied only if the current address belongs to a segment
#[allow(unused)]
pub const PLUGIN_SEG: i32 = 0x0004;

/// Unload the plugin immediately after calling 'run'
#[allow(unused)]
pub const PLUGIN_UNL: i32 = 0x0008;

/// Plugin should not appear in the Edit, Plugins menu
#[allow(unused)]
pub const PLUGIN_HIDE: i32 = 0x0010;

/// A debugger plugin
#[allow(unused)]
pub const PLUGIN_DBG: i32 = 0x0020;

/// Load plugin when a processor module is loaded
#[allow(unused)]
pub const PLUGIN_PROC: i32 = 0x0040;

/// Load plugin when IDA starts and keep it in memory
#[allow(unused)]
pub const PLUGIN_FIX: i32 = 0x0080;

/// The plugin can work with multiple idbs in parallel
#[allow(unused)]
pub const PLUGIN_MULTI: i32 = 0x0100;

/// Scripted plugin
#[allow(unused)]
pub const PLUGIN_SCRIPTED: i32 = 0x8000;

#[allow(unused)]
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct range_t {
    pub start: ea_t,
    pub end: ea_t,
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct func_t {
    pub range: range_t,
    pub flags: u64,
}

unsafe impl ExternType for func_t {
    type Kind = autocxx::cxx::kind::Opaque;
    type Id = type_id!("func_t");
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct plugin_t {
    pub version: i32, // Should be equal to IDP_INTERFACE_VERSION
    pub flags: i32,   // Plugin feature flags
    pub init: Option<extern "C" fn() -> *const plugmod_t>, // Initialize plugin
    pub term: Option<extern "C" fn()>, // Terminate plugin
    pub run: Option<extern "C" fn(arg: usize) -> bool>, // Invoke plugin
    pub comment: *const std::ffi::c_char, // Long comment about the plugin
    pub help: *const std::ffi::c_char, // Multiline help about the plugin
    pub wanted_name: *const std::ffi::c_char, // The preferred short name of the plugin
    pub wanted_hotkey: *const std::ffi::c_char, // The preferred hotkey to run the plugin
}
unsafe impl Sync for plugin_t {}
