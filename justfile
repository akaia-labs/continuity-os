# curl -sSf https://install.spacetimedb.com | sh

setup:
	(mise trust)
	(mise install)
	(cargo install wasm-opt)

local-publish:
	(spacetime publish -s localhost --project-path server crowd)

local-dbcall:
	(spacetime call -s localhost crowd)

local-dblogs:
	(spacetime logs -s localhost crowd)

generate-module_bindings:
	spacetime generate --lang rust \
		--project-path packages/crownest \
		--out-dir packages/crowlink/src/common/clients/crownest/_generated/module_bindings

generate: generate-module_bindings
	(echo "DONE.")

crowlink-devserver:
	(cd packages/crowlink)
	(cargo run)
