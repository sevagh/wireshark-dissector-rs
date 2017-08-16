WORKSPACES := common server client transform

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

.PHONY: clean, build, lint
