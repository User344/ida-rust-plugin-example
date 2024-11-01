use crate::ida::bridge;
use autocxx::subclass::*;

// `ffi` module is required for `subclass` macro to work.
mod ffi {
    pub use crate::ida::bindgen::*;
}

#[subclass]
#[derive(Default)]
pub struct ExamplePlugin;

// You can override constructor too, if needed.
// impl ExamplePlugin {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// Need to implement this so we can allocate "self-owned" object,
// which can be deallocated using `delete_self` method later,
// when virtual destructor is called.
impl CppSubclassSelfOwned<ffi::ExamplePluginCpp> for ExamplePlugin {}

impl ffi::plugmod_t_methods for ExamplePlugin {
    fn run(&mut self, arg: usize) -> bool {
        bridge::msg(&format!("Running ExamplePlugin, arg={}\n", arg));

        let ea = ffi::get_screen_ea();
        let func = ffi::get_func(ea);
        if func.is_null() {
            bridge::msg("No function found at the current screen address\n");
            return true;
        }

        let func = unsafe { &*func };
        bridge::msg(&format!("Screen ea={:#x}, func=\n{:#x?}\n", ea.0, func));

        true
    }
}

impl Drop for ExamplePlugin {
    fn drop(&mut self) {
        bridge::msg("Dropping ExamplePlugin\n");
        self.delete_self();
    }
}
