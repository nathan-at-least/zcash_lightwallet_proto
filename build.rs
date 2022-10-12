fn main() -> std::io::Result<()> {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src")
        .include("subtree/lightwalletd/walletrpc")
        .input("subtree/lightwalletd/walletrpc/compact_formats.proto")
        .input("subtree/lightwalletd/walletrpc/darkside.proto")
        .input("subtree/lightwalletd/walletrpc/service.proto")
        .run()
}
