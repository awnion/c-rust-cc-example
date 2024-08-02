# libsum:
#         cd c && clang -shared -o libsum.so -fPIC main.c

# buildr: libsum
#         #!/bin/bash
#         RUSTFLAGS="-v -L c -l`pwd`/c/libsum.so" cargo run

clean:
        rm -rf c/libsum.so
        rm -rf target
