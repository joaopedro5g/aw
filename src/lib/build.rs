// src/build.rs
fn main() {
  cc::Build::new()
   .cpp(true)
   .file("src/main.cpp")
   .compile("main.a");
  }