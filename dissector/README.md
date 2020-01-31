### Sources

* https://github.com/alexcrichton/rust-ffi-examples/tree/master/c-to-rust
* http://www.protomatics.com/wireshark_dissector.html (without the weird TSN thingy)
* https://www.wireshark.org/docs/wsdg_html_chunked/ChDissectAdd.html
* https://www.wireshark.org/docs/wsar_html/epan/structtvbuff.html


### Instructions
Download and untar the [Wireshark source code](https://www.wireshark.org/download.html). Only tested on Wireshark 3.2.1 for now.

We assume that `wireshark-3.2.1` is located in `~/wireshark-3.2.1/` and `wireshark-dissector-rs` is located in `~/wireshark-dissector-rs`.


```shell
$ cd ~/wireshark-dissector-rs
$ cd dissector/plugins/dummy/ && make -f ./Makefile.rust && cd -
$ cp -r ./dissector/plugins/dummy ~/wireshark-3.2.1/plugins/epan/
$ cd ~/wireshark-3.2.1
$ mkdir build
$ cd ~/wireshark-3.2.1/build
# include our plugin directory
$ cmake .. -DCUSTOM_PLUGIN_SRC_DIR="plugins/epan/dummy"
$ make
$ sudo make install
```

After the long first compile where `wireshark` is built, simply rebuilding the plugins is enough if you change your dissector later:

```shell
$ cd ~/wireshark-3.2.1/build
$ cp -r ./dissector/plugins/dummy ~/wireshark-3.2.1/plugins/epan/
$ make plugins
$ sudo make install
```

### Development Notes
There are several resources available:

* Instructions of plugin development: https://github.com/wireshark/wireshark/blob/master/doc/README.plugins
* Example of a simple plugin: https://github.com/wireshark/wireshark/tree/master/doc/plugins.example
* More complex plugins: https://github.com/wireshark/wireshark/tree/master/plugins/epan

You may need to re-generate `plugin.c` when you edit `packet-dummy.c` or `packet-dummy.h`:

```shell
$ cd ~/wireshark-dissector-rs/dissector/plugins/dummy/
$ ~/wireshark-3.2.1/tools/make-plugin-reg.py \
        # args: 
        # 1st: /path/to/wireshark-dissector-rs/dissector/plugins/dummy/
        # 2nd: plugin
        # rest: list of your source codes
        $PWD plugin packet-dummy.c packet-dummy.h
```