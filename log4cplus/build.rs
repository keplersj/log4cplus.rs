use conan2::ConanInstall;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");

    let metadata = ConanInstall::new()
        .build("missing")
        .detect_profile()
        .run()
        .parse();

    cxx_build::bridge("src/lib.rs") // returns a cc::Build
        .includes(metadata.include_paths())
        .include("src")
        // Minimum needed for contructor shim in shim.hpp
        .std("c++14")
        // Building without Unicode/widestring support is inexplicable broken.
        // For now, this isn't a big deal because all Rust strings are UTF-8/Unicode by default
        //  and our lightweight shim over `log4cplus::tstring` handles it seemlessly.
        //
        // In the future it may be worth following `fish-shell`'s lead and explicitly supporting `widestring` with binding on a `CxxWideString`
        // Ref: <https://github.com/fish-shell/fish-shell/blob/master/doc_internal/rust-devel.md>
        // Ref: <https://github.com/fish-shell/cxx/blob/fish/src/cxx_wstring.rs>
        .flag("-DUNICODE")
        .compile("log4cplus-bridge");

    metadata.emit();
}