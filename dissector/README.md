### Sources

* https://github.com/alexcrichton/rust-ffi-examples/tree/master/c-to-rust
* http://www.protomatics.com/wireshark_dissector.html (without the weird TSN thingy)
* https://www.wireshark.org/docs/wsdg_html_chunked/ChDissectAdd.html
* https://www.wireshark.org/docs/wsar_html/epan/structtvbuff.html

### Instructions

Download and untar the [Wireshark source code](https://www.wireshark.org/download.html). Only tested on Wireshark 2.4.0 for now.

```shell
$ pwd
wireshark-dissector-rs
$ cd dissector/plugins/dummy/ && make -f ./Makefile.rust && cd -
$ cp -r ./dissector/plugins /path/to/wireshark-2.4.0
$ cd /path/to/wireshsark-2.4.0
$ ./autogen.sh
$ make -C plugins
$ make
$ sudo make install
```

After the long first compile where `wireshark` is built, simply rebuilding the plugins is enough if you change your dissector later:

```shell
$ ./autogen.sh
$ make -C plugins
```

Also, you must modify `wireshark-2.4.0/epan/Makefile.am` to include `dummy`:

```shell
if ENABLE_STATIC
-include ../plugins/Custom.make
plugin_src = \
        ...
        ../plugins/gryphon/packet-gryphon.c \
        ../plugins/gryphon/plugin.c \
        ../plugins/dummy/plugin.c \
        ../plugins/dummy/packet-dummy.c \
        ...
```

Without this step (although I couldn't find it in any online tutorials), `wireshark` fails to start with `unrecognized symbol` (i.e. couldn't link to `dummy.so`).
