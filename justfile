# curl -sSf https://install.spacetimedb.com | sh

setup:
	(mise trust)
	(mise install)
	(cargo install wasm-opt)


unsafe-local-drop:
	(spacetime delete -s localhost crowspace)

local-publish-crowspace:
	(spacetime publish -s localhost --project-path modules/crowspace crowspace)

local-dbcall-crowspace:
	(spacetime call -s localhost crowspace)

local-dblogs-crowspace:
	(spacetime logs -s localhost crowspace)


generate-module_bindings:
	spacetime generate --lang rust \
		--project-path modules/crowspace \
		--out-dir packages/crowcomm/src/modules/crowspace/_generated/module_bindings


local-publish: local-publish-crowspace
	(echo "DONE.")


generate: generate-module_bindings
	(echo "DONE.")


telecrow-dev:
	(cd services/telecrow && cargo run)

telecrow-inspect:
	(cd services/telecrow && RUST_LOG=trace cargo run)


chatter-dev:
	(cd applications/chatter && cargo run)
