# picotcp-sys
Rust binding for picotcp

## How to compile

```
sudo aptitude install libvdeplug-dev
cargo build
```

If you have an error similar to this:

```
thread 'main' panicked at 'Failed to generate bindings for /home/user/picotcp-sys/target/debug/build/picotcp-sys-cd06b602a1ad2b2a/out/picotcp/include/pico_dev_vde.h: ()', src/
libcore/result.rs:799
```

Try compiling the file with `clang`, you will get a nicer error:

```
$ clang build/include/pico_dev_vde.h
build/include/pico_dev_vde.h:11:10: fatal error: 'libvdeplug.h' file not found
#include <libvdeplug.h>
         ^
1 error generated.
```
