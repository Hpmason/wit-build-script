# Wit bindgen build scripts
`wasmtime` makes use of macros both in for their component system and their `wit-bindgen` crate to generate bindings from .wit files. The use of the proc-macro to read/parse from the file system has a several drawbacks, including recompiling and poor readbility of generated code. 

This repo contain crates to generate these bindings from build.rs scripts. Using build.rs scripts lets your see the generated code and gives you control on when the bindings should be recompiled.

## Status
This repo is still very much in testing/prototyping phase, so expect changes. This repo is open to questions/suggestions for improvements!
