use std::path::Path;

const PROTO_DIR: &str = "subtree/lightwalletd/walletrpc";
const OUT_DIR: &str = "target/gensrc";

fn main() -> std::io::Result<()> {
    let protodir = std::path::Path::new(PROTO_DIR);
    let outdir = std::path::Path::new(OUT_DIR);

    recreate_dir(outdir)?;

    let mut libmod = std::fs::File::create(outdir.join("lib.rs"))?;

    let mut cg = protobuf_codegen_pure::Codegen::new();
    cg.out_dir(outdir);
    cg.include(protodir);

    for res in protodir.read_dir()? {
        let dirent = res?;
        if dirent.file_type()?.is_file() {
            let osfn = dirent.file_name();
            let filename = osfn.to_str().expect("non-utf8 filename");

            if let Some(modname) = filename.strip_suffix(".proto") {
                use std::io::Write;

                let protopath = dirent.path();
                eprintln!("Translating {:?}...", protopath.display());
                cg.input(protopath);

                writeln!(libmod, "pub mod {modname};")?;
            }
        }
    }

    cg.run()
}

fn recreate_dir(d: &Path) -> std::io::Result<()> {
    std::fs::remove_dir_all(d).or_else(|e| {
        use std::io::ErrorKind::NotFound;

        match e.kind() {
            // If there's no outdir, this is ok:
            NotFound => Ok(()),
            _ => Err(e),
        }
    })?;

    std::fs::create_dir(d)
}
