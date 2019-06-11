fn main() -> Result<(), std::io::Error> {
    let dest = std::path::PathBuf::from(
        &std::env::var("OUT_DIR").expect("PathBuf failed")
    );

    println!("cargo:rerun-if-changed=build.rs");

    let mut file = std::fs::File::create(&dest.join("gl_bindings.rs"))?;

    gl_generator::Registry::new(
        gl_generator::Api::Gles2,
        (3, 3),
        gl_generator::Profile::Core,
        gl_generator::Fallbacks::All,
        []
    ).write_bindings(gl_generator::StructGenerator, &mut file)?;

    Ok(())
}
