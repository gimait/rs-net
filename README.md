# rs-net

This repo includes a minimal implementation to communicate a client-server application with gRPC.

The gRPC client has been implemented in Rust, following the `Tonic` crate's tutorial, while the server runs in .NET Framework 4.8, following the grpc examples for C#.

So far, to run this the FrameworkServer and Rust binaries need to be compiled and run individually.
To do so, run:

```
dotnet build FrameworkServer
cargo build
```

Then you can run the binaries generated in separate terminals.
