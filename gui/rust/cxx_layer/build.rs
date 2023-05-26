
fn main() {
    //CxxQtBuilder::new().file("src/lib.rs").build();
    cxx_build::bridge("src/client_cxx.rs").compile("client_cxx");
    cxx_build::bridge("src/lib.rs").compile("lib");
    cxx_build::bridge("src/library_cxx.rs").compile("library_cxx");
}