
fn main() {
    //CxxQtBuilder::new().file("src/model").build();
    cxx_build::bridge("src/client_cxx.rs").compile("client_cxx");
    cxx_build::bridge("src/lib.rs").compile("lib");
    cxx_build::bridge("src/library_cxx.rs").compile("library_cxx");
    cxx_build::bridge("../../../../library/src/epub_cxx.rs").compile("epub");
    
}