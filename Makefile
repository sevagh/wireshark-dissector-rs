WORKSPACES := common server client transform
WIRESHARK_PLUGIN_DIR := dissector/plugins

all: lint

lint:
	$(foreach workspace,$(WORKSPACES),\
	    cd $(workspace) &&\
	    cargo +nightly fmt &&\
	    cargo +nightly clippy &&\
	    cd - \
	    ;)

build:
	$(foreach workspace,$(WORKSPACES),\
	    cd $(workspace) &&\
	    cargo build $(CARGO_FLAGS) &&\
	    cd - \
	    ;)

clean:
	$(foreach workspace,$(WORKSPACES),\
	    cd $(workspace) &&\
	    cargo clean &&\
	    cd - \
	    ;)

wireshark_plugin:
	@if test -z "$$WIRESHARK_SRC_DIR"; then echo "Please define WIRESHARK_SRC_DIR" && exit -1; fi;
	@cd $(WIRESHARK_PLUGIN_DIR)/dummy && make -f ./Makefile.rust
	@cp -r $(WIRESHARK_PLUGIN_DIR) $$WIRESHARK_SRC_DIR
	@cd $$WIRESHARK_SRC_DIR && ./autogen.sh && make -C plugins && make && sudo make install && cd -


.PHONY: clean, build, lint
