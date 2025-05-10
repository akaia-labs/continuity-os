# `curl -sSf https://install.spacetimedb.com | sh` ?
setup:
    (mise trust)
    (mise install)
    (cargo install wasm-opt)


#* DEVELOPMENT

corvid-dev:
    (cd subsystems/corvi.d && ./rvvm/rvvm_arm64)

telecrow-dev:
    (cd services/telecrow && cargo run)

telecrow-inspect:
    (cd services/telecrow && RUST_LOG=trace cargo run)

jayterm-dev:
    (cd userspace/applications/jayterm && cargo run)

generate-module_bindings:
    spacetime generate --lang rust \
    	--project-path modules/corvidx \
    	--out-dir packages/crowdcomm_sdk/src/modules/corvidx/common/_generated_bindings

generate: generate-module_bindings
    (echo "✅ DONE.")


#* TESTS

corvutils-test:
    (cd libraries/corvutils && cargo test)

corvutils-test-dbg:
    (cd libraries/corvutils && cargo test -- --show-output)


#* DATABASE ADMINISTRATION

unsafe-local-corvidx-drop:
    (spacetime delete -s localhost corvidx)

local-corvidx-publish:
    (spacetime publish -s localhost --project-path modules/corvidx corvidx)

local-corvidx-call:
    (spacetime call -s localhost corvidx)

local-corvidx-log:
    (spacetime logs -s localhost -f corvidx)

local-publish: local-corvidx-publish
    (echo "✅ DONE.")

unsafe-local-republish: unsafe-local-corvidx-drop
    (just local-publish)
    (echo "✅ DONE.")
