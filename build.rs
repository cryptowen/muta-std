fn main() {
    cc::Build::new()
        .file("src/asm/syscall.S")
        .compile("muta-syscall");
}
