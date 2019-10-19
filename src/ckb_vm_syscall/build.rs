use std::fs;
use std::env;
use std::process::Command;

static AS: &'static str = "riscv32-unknown-elf-as";
static AR: &'static str = "riscv32-unknown-elf-ar";

fn main() {
    check_target();
    if have_tools() {
        build();
        copy_output_to_srcbin();
    } else {
        copy_srcbin_to_output();
    }
    emit_linker_flags();
}

fn check_target() {
    if let Ok(target) = env::var("TARGET") {
        if target != "riscv32imac-unknown-none-elf" {
            panic!("unsupported target: {}", target);
        }
    } else {
        panic!("unable to get target for RISC-V syscall stubs");
    }
}

fn have_tools() -> bool {
    let mut as_cmd = Command::new(AS);
    as_cmd.arg("--version");
    let r = as_cmd.status().ok().map(|s| s.success());
    if r != Some(true) {
        return false;
    }

    let mut ar_cmd = Command::new(AR);
    ar_cmd.arg("--version");
    let r = ar_cmd.status().ok().map(|s| s.success());
    if r != Some(true) {
        return false;
    }

    true
}

fn manifest_dir() -> String {
    env::var("CARGO_MANIFEST_DIR").expect("unable to get manifest dir")
}

fn out_dir() -> String {
    env::var("OUT_DIR").expect("unable to get output dir")
}

struct Files {
    src: String,
    out_o: String,
    out_a: String,
    src_a: String,
}

fn files() -> Files {
    Files {
        src: format!("{}/{}", manifest_dir(), "ecall32.s"),
        out_o: format!("{}/{}", out_dir(), "ecall32.o"),
        out_a: format!("{}/{}", out_dir(), "libecall32.a"),
        src_a: format!("{}/{}", manifest_dir(), "libecall32.a"),
    }
}

fn build() {
    let files = files();

    let mut as_cmd = Command::new(AS);
    as_cmd.arg("-march=rv32imac");
    as_cmd.arg(&files.src);
    as_cmd.arg("-o");
    as_cmd.arg(&files.out_o);

    let r = as_cmd.status().expect("running as");
    assert!(r.success(), "running as failed");

    let mut ar_cmd = Command::new(AR);
    ar_cmd.arg("-rsv");
    ar_cmd.arg(&files.out_a);
    ar_cmd.arg(&files.out_o);

    let r = ar_cmd.status().expect("running ar");
    assert!(r.success(), "running as failed");
}

fn copy_output_to_srcbin() {
    let files = files();
    fs::copy(&files.out_a, &files.src_a).expect("copying archive");
}

fn copy_srcbin_to_output() {
    let files = files();
    fs::copy(&files.src_a, &files.out_a).expect("copying archive");
}

fn emit_linker_flags() {
    println!("cargo:rustc-link-search=native={}", out_dir());
    println!("cargo:rustc-link-lib=static=ecall32");
}
