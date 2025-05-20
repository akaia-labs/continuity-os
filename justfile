# `curl -sSf https://install.spacetimedb.com | sh` ?
setup:
    (mise trust)
    (mise install)
    (cargo install wasm-opt)


#* DEVELOPMENT

corvidx-generate:
    spacetime generate --lang rust \
    	--project-path modules/corvidx/server \
    	--out-dir modules/corvidx/client/src/common/stdb/generated_bindings

generate: corvidx-generate
    (echo "✅ DONE.")

telecrow-dev:
    (cd subsystem/corvi.d/services/telecrow && cargo run)

telecrow-inspect:
    (cd subsystem/corvi.d/services/telecrow && RUST_LOG=trace cargo run)

jayterm-dev:
    (cd userspace/apps/jayterm && cargo run)


#* TESTS

corvutils-test:
    (cd libraries/corvutils && cargo test)

corvutils-test-dbg:
    (cd libraries/corvutils && cargo test -- --show-output)


#* DATABASE ADMINISTRATION

unsafe-local-corvidx-drop:
    (spacetime delete -s localhost corvidx)
    (echo "✅ DONE.")

local-corvidx-publish:
    (spacetime publish -s localhost --project-path modules/corvidx/server corvidx)

local-corvidx-call:
    (spacetime call -s localhost corvidx)

local-corvidx-log:
    (spacetime logs -s localhost -f corvidx)

local-publish: local-corvidx-publish
    (echo "✅ DONE.")

unsafe-local-republish: unsafe-local-corvidx-drop
    (just local-publish)
