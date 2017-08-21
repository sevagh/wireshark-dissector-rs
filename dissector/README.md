### Sources

* https://github.com/alexcrichton/rust-ffi-examples/tree/master/c-to-rust
* http://www.protomatics.com/wireshark_dissector.html (without the weird TSN thingy)
* https://www.wireshark.org/docs/wsdg_html_chunked/ChDissectAdd.html
* https://www.wireshark.org/docs/wsar_html/epan/structtvbuff.html

### Instructions

Download and untar the [Wireshark source code](https://www.wireshark.org/download.html). Only tested on Wireshark 2.4.0 for now.

```shell
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
