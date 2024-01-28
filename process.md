
## reqs...
* user needs to have clangd12
* USER CONFIGURATION is a config.yaml, see clangd.llvm.org/config
* see guides/remote-index: .yaml file needs to have an If for path matching, and then takes some Index variables
* then docker container needs to actually run an indexer with `clang-index-server`
* docker also needs to expose a port

```
To build remote-index-enabled clangd and clangd-index-server, you need:

gRPC libraries (e.g. apt install libgrpc++-dev libprotobuf-dev protobuf-compiler-grpc or brew install grpc protobuf or build from source)
to set the -DCLANGD_ENABLE_REMOTE=On and possibly -DGRPC_INSTALL_PATH CMake flags
```

For latest clangd:
```
https://github.com/clangd/clangd/releases/tag/17.0.3
```


## workflow then
* user runs `easyinclude init`. user needs to specify a running container that has an exposed port (and what that port is)
* also need some sort of path specification. maybe some intelligent path logic to find instances of `compile_commands.json files`
* `easyinclude` then does a docker exec to run the indexer on the port. 
* now append lines to user .yaml file for cleanup on `easyinclude deinit`.

## open questions
need to think about paths, mounts, and ports. wouldn't be a bad thing to do in a .yaml file. something like:

```
image_name: my_image
exposed_port: 6900
mount:
```

Need to expose the mount mapping for damn sure!! Docker path : User path or something like that

need to verify clangd both locally and in the container so that's some work 🍳


How to differentiate between:
User config file is /Users/thomas/Library/Preferences/clangd/config.yaml

And whatever nvim does??
Ok nvm `nvim` seems to call the same shit



 ~/clangd_17.0.3/bin/clangd-index-server --server-address 127.0.0.1:50051 --log=verbose proj.idx /home/root/libmodal-cv

Users will also need to:
Building/releases#
The client and server require the gRPC libraries. Because of this dependency, they are not enabled by default in CMake.

To build remote-index-enabled clangd and clangd-index-server, you need:

gRPC libraries (e.g. apt install libgrpc++-dev libprotobuf-dev protobuf-compiler-grpc or brew install grpc protobuf or build from source)
to set the -DCLANGD_ENABLE_REMOTE=On and possibly -DGRPC_INSTALL_PATH CMake flags
The clangd releases on GitHub include remote index support, but official LLVM releases do not (yet).
