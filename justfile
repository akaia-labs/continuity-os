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
