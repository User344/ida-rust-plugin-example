use autocxx::prelude::*;
use cxx::let_cxx_string;

include_cpp! {
    #include "bridge.hpp"
    name!(bridge_ffi)
    safety!(unsafe)
    generate!("bridge_msg")
}

pub fn msg(format: &str) {
    let_cxx_string!(str = format);
    bridge_ffi::bridge_msg(&str);
}
