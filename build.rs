fn main() {
    pkg_config::Config::new()
        .atleast_version("1.2")
        .probe("z")
        .unwrap();
    let src = [
        "src/amdmasync.cpp",
        "src/atv.cpp",
        "src/dma.cpp",
        "src/dsp.cpp",
        "src/fmdmasync.cpp",
        "src/fskburst.cpp",
        "src/gpio.cpp",
        "src/iqdmasync.cpp",
        "src/mailbox.c",
        "src/ngfmdmasync.cpp",
        "src/ookburst.cpp",
        "src/phasedmasync.cpp",
        "src/raspberry_pi_revision.c",
        "src/rpi.c",
        "src/serialdmasync.cpp",
        "src/util.cpp",
    ];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("include")
        .flag("-O3")
        .flag("-std=c++11")
        .flag("-Wall")
        .flag("-Wno-unused-variable")
        .flag("-fPIC")
        .define("USE_ZLIB", None);
    build.compile("rpitx");
}
