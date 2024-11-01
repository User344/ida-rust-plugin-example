use crate::ExamplePlugin;
use autocxx::prelude::*;

include_cpp! {
    #include "ida.hpp"
    #include "idp.hpp"
    #include "loader.hpp"
    #include "kernwin.hpp"
    #include "funcs.hpp"
    name!(bindgen_ffi)
    safety!(unsafe)
    extern_cpp_opaque_type!("func_t", crate::ida::types::func_t)
    generate!("get_screen_ea")
    generate!("get_func")
    generate!("plugmod_t")
    subclass!("plugmod_t", ExamplePlugin)
}

pub use bindgen_ffi::*;
