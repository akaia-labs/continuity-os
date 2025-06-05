set dotenv-load
set positional-arguments


# `curl -sSf https://install.spacetimedb.com | sh` ?
setup:
    (mise trust)
    (mise install)
    (cargo install wasm-opt)


#* DEVELOPMENT

corvidx-generate:
    spacetime generate --lang rust \
    	--project-path core/stdb/modules/corvidx/server \
    	--out-dir core/stdb/modules/corvidx/client/src/common/stdb/generated_bindings

generate: corvidx-generate
    (echo "✅ DONE.")

telecrow-dev:
    (cd subsystems/communication/services/telecrow && cargo run)

telecrow-inspect:
    (cd subsystems/communication/services/telecrow && RUST_LOG=trace cargo run)

jayterm-dev:
    (cd applications/jayterm && cargo run)


#* TESTS

corvutils-test:
    (cd packages/corvutils && cargo test)

corvutils-test-dbg:
    (cd packages/corvutils && cargo test -- --show-output)


#* DATABASE ADMINISTRATION

unsafe-local-corvidx-drop:
    (spacetime delete -s localhost $CORVID_MODULES_CORE_DBNAME)
    (echo "✅ DONE.")

local-corvidx-publish:
    (spacetime publish -s localhost --project-path core/stdb/modules/corvidx/server $CORVID_MODULES_CORE_DBNAME)

local-corvidx-call:
    (spacetime call -s localhost $CORVID_MODULES_CORE_DBNAME)

local-corvidx-sql *args='':
    (spacetime sql -s localhost $CORVID_MODULES_CORE_DBNAME "$@")

local-corvidx-subscribe *args='':
    (spacetime subscribe -s localhost $CORVID_MODULES_CORE_DBNAME "$@")

local-corvidx-log:
    (spacetime logs -s localhost -f $CORVID_MODULES_CORE_DBNAME)

local-publish: local-corvidx-publish
    (echo "✅ DONE.")

unsafe-local-republish: unsafe-local-corvidx-drop
    (just local-publish)
