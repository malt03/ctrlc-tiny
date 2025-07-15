use std::{env, fmt::Display, io, path::PathBuf};

#[derive(Debug)]
enum Error {
    BindgenError(bindgen::BindgenError),
    IoError(io::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BindgenError(e) => write!(f, "Bindgen error: {}", e),
            Error::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}
impl From<bindgen::BindgenError> for Error {
    fn from(err: bindgen::BindgenError) -> Self {
        Error::BindgenError(err)
    }
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

fn main() -> Result<(), Error> {
    cc::Build::new().file("c_src/sigint.c").compile("sigint");

    let bindings = bindgen::Builder::default()
        .header("c_src/sigint.h")
        .allowlist_function("init_sigint_handler")
        .allowlist_function("get_is_sigint_received")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("sigint_bindings.rs"))?;

    Ok(())
}
