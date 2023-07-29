fn main() {
    cxx_build::bridge("src/epub_cxx.rs").compile("epub_cxx");
    cxx_build::bridge("src/lib.rs").compile("lib");

}