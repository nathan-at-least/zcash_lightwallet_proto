A thin-as-possible rust bindings wrapper for the Zcash [lightwalletd](https://github.com/zcash/lightwalletd) gRPC protocol.

# Status

:warning: ⚠ This is a proof-of-concept prototype and should not be used in production.

This repository is a proof of concept at creating a minimal modular rust bindings crate for the Lightwalletd protocol. The [librustzcash](https://github.com/zcash/librustzcash/issues/585) project may integrate grpc bindings directly which would be better maintained and vetted for production use in the future, see [librustzcash issue #585](https://github.com/zcash/librustzcash/issues/585).

This crate depends on `protobuf-codegen-pure` which will be imminently superceded by `protobug-codegen` according to [the docs](https://docs.rs/protobuf-codegen-pure/latest/protobuf_codegen_pure/#version-2).

# Build

This crate relies on `protobuf-codegen-pure` to generate bindings without requiring any build toolchains outside of `cargo` (namely there's no requirement for `protoc` to be installed).

So a standard `cargo build` should work.

## Build Design

The `.proto` files are imported directly from [lightwalletd](https://github.com/zcash/lightwalletd) as a git subtree within the `./subtree` directory. This design is intended to enable safe tracking of upstream changes by updating the subtree without other manual copying or editing of `.proto` files.

The `build.rs` script uses `protobuf-codegen-pure` to generate rust mods, writing them into `$OUT_DIR/generated-source/`. Additionally a `container.rs` source file is written with `pub mod ${MODNAME};` for each `.proto` generated rust mod.

The `lib.rs` directly `include!()`'s the `container.rs`, which pulls in each generated submod.

Unfortunately API docs are not produced from the upstream `.proto` files with this current design. A nice improvement would be to ensure rust API docs are automatically generated from upstream `.proto` API documentation.
