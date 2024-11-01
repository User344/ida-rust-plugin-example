# ida-rust-plugin-example

Recently I needed to write a plugin for IDA Pro, and I wanted to do it in Rust.
Unfortunately, I couldn't find any good starting points or templates for that.
That's how this example came to be.

## Building

Before running `cargo build` you need to specify the path to IDA SDK
in `IDA_SDK_DIR` environment variable, or hardcode it in [build.rs](build.rs).

## Autocxx

This example uses [autocxx](https://github.com/google/autocxx) to automagically generate
Rust bindings for IDA Pro. It is an awesome tool but unfortunately
it is rather experimental yet and has some bugs.
Please make sure you read the [autocxx docs](https://google.github.io/autocxx/)!

> [!WARNING]  
> Autocxx has a bug!
> If you look at [build.rs](build.rs) you will see that we don't use
> latest cargo version of autocxx but a specific commit.
> This is due to a [bug](https://github.com/google/autocxx/issues/1384)
> in autocxx-build that prevents us from
> creating builder multiple times. The fix is already merged, but
> it is not released yet.

## Points of interest

### [build.rs](build.rs)

Build script is used to:

- Link IDA SDK static library.
- Use autocxx to generate Rust bindings.
- Use autocxx to generate bindings for [bridge](#src/ida/bridge.rs).
- Compile and link bridge.

### [src/ida/bindgen.rs](src/ida/bindgen.rs)

File that contains all of the bindings for IDA SDK.
This is where you want to add all the types and methods you want to use.

### [src/ida/bridge.rs](src/ida/bridge.rs)

Sometimes a method you want to call can not be generated by autocxx.
In this case you could either define it manually using [cxx::bridge](https://google.github.io/autocxx/workflow.html#mixing-manual-and-automated-bindings),
or you could write your own method that wraps it and exposes it using bridge.

Just declare your method in [bridge.hpp](src/ida/bridge.hpp),
implement it in [bridge.cpp](src/ida/bridge.cpp), and include it in [bridge.rs](srcidabridgers).

### [src/ida/types.rs](src/ida/types.rs)

While autocxx can generate some simple structs and enums, most of them
are not simple and would require manual reimplementation.
Luckily for us, doing so in Rust is pretty easy.

### [src/lib.rs](src/lib.rs)

Just like in a regular IDA plugin, we need to export information about our plugin,
and some methods like `init`. That's where I do it, but of course you can do it
in any other place.

### [src/plugin.rs](src/plugin.rs)

This is where we override `plugmod_t` methods for a modern IDA plugin.
