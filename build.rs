extern crate gl_generator;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new(&"src").join("gl.rs");

    if !path.exists() {
        let mut file = File::create(&path).unwrap();

        Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
            .write_bindings(GlobalGenerator, &mut file)
            .unwrap();
    }
}
