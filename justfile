# curl -sSf https://install.spacetimedb.com | sh

setup:
	(mise trust)
	(mise install)
	(cargo install wasm-opt)


local-publish-crowchat:
	(spacetime publish -s localhost --project-path modules/crowchat crowchat)


local-dbcall-crowchat:
	(spacetime call -s localhost crowchat)


local-dblogs-crowchat:
	(spacetime logs -s localhost crowchat)


generate-module_bindings:
	spacetime generate --lang rust \
		--project-path modules/crowchat \
		--out-dir packages/crowtocol_rs/src/modules/crowchat/_generated/module_bindings


local-publish: local-publish-crowchat
	(echo "DONE.")


generate: generate-module_bindings
	(echo "DONE.")


telecrow-dev:
	(cd services/telecrow && cargo run)

telecrow-inspect:
	(cd services/telecrow && RUST_LOG=trace cargo run)


chatter-dev:
	(cd applications/chatter && cargo run)
