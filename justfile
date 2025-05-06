subsystem-dev:
	(cd subsystem && ./rvvm/rvvm_arm64)


# `curl -sSf https://install.spacetimedb.com | sh` ?
setup:
	(mise trust)
	(mise install)
	(cargo install wasm-opt)


unsafe-local-drop:
	(spacetime delete -s localhost crowd-core)

local-publish-core:
	(spacetime publish -s localhost --project-path modules/crowd_core crowd-core)

local-dbcall-core:
	(spacetime call -s localhost crowd-core)

local-dblogs-core:
	(spacetime logs -s localhost crowd-core)


generate-module_bindings:
	spacetime generate --lang rust \
		--project-path modules/crowd_core \
		--out-dir packages/crowcomm/src/modules/crowd_core/common/_generated_bindings


local-publish: local-publish-core
	(echo "✅ DONE.")

unsafe-local-republish: unsafe-local-drop
	(just local-publish)


generate: generate-module_bindings
	(echo "✅ DONE.")


telecrow-dev:
	(cd services/telecrow && cargo run)

telecrow-inspect:
	(cd services/telecrow && RUST_LOG=trace cargo run)


chatter-dev:
	(cd applications/chatter && cargo run)
