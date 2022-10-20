use std::path::Path;

fn main() -> std::io::Result<()> {
    let outdir = get_required_env("OUT_DIR")?;

    let protodir = Path::new("subtree/lightwalletd/walletrpc");
    let gensrc = Path::new(&outdir).join("generated-source");
    recreate_dir(&gensrc)?;

    let mut codegen = protobuf_codegen_pure::Codegen::new();
    codegen
        .out_dir(&gensrc)
        .include("subtree/lightwalletd/walletrpc");

    let mut containermod = std::fs::File::create(gensrc.join("container.rs"))?;

    for res in std::fs::read_dir(protodir)? {
        let dirent = res?;
        if dirent.file_type()?.is_file() {
            let filename = dirent.file_name();
            if let Some(modname) = filename.to_str().and_then(|s| s.strip_suffix(".proto")) {
                use std::io::Write;

                codegen.input(protodir.join(&filename));
                writeln!(containermod, "pub mod {};", modname)?;
            }
        }
    }

    codegen.run()?;
    Ok(())
}

/// Get an environment variable or else return a `std::io::Error`
fn get_required_env(name: &str) -> std::io::Result<String> {
    use std::io::{Error, ErrorKind::Other};

    std::env::var(name).map_err(|e| Error::new(Other, format!("{}: {}", name, e)))
}

/// Remove a directory and all contents, if it exists, then create an empty directory at that path
fn recreate_dir(dir: &Path) -> std::io::Result<()> {
    std::fs::remove_dir_all(dir).or_else(|e| {
        use std::io::ErrorKind::NotFound;

        if e.kind() == NotFound {
            // This is okay; our goal is to create a new empty directory.
            Ok(())
        } else {
            Err(e)
        }
    })?;

    std::fs::create_dir(dir)?;
    Ok(())
}
