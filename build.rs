fn main() {
    cc::Build::new()
        .file("c/eint.c")
        .static_flag(true)
        .flag("-O3")
        .flag("-Wall")
        .flag("-Werror")
        .compile("eint-c-impl");
}
