set dotenv-load
set positional-arguments


# `curl -sSf https://install.spacetimedb.com | sh` ?
setup:
    (mise trust)
    (mise install)
    (cargo install wasm-opt)


#* DEVELOPMENT

singularity-generate:
    spacetime generate --lang rust \
    	--project-path core/spacetimedb/modules/singularity/server \
    	--out-dir core/spacetimedb/modules/singularity/client/src/common/stdb/generated_bindings

generate: singularity-generate
    (echo "✅ DONE.")

telecrow-dev:
    (cd subsystems/corvi.d/services/telecrow && cargo run)

telecrow-inspect:
    (cd subsystems/corvi.d/services/telecrow && RUST_LOG=trace cargo run)

jayterm-dev:
    (cd applications/jayterm && cargo run)


#* TESTS

corvutils-test:
    (cd packages/corvutils && cargo test)

corvutils-test-dbg:
    (cd packages/corvutils && cargo test -- --show-output)


#* DATABASE ADMINISTRATION

unsafe-local-singularity-drop:
    (spacetime delete -s localhost singularity)
    (echo "✅ DONE.")

local-singularity-publish:
    (spacetime publish -s localhost --project-path core/spacetimedb/modules/singularity/server singularity)

local-singularity-call:
    (spacetime call -s localhost singularity)

local-singularity-sql *args='':
    (spacetime sql -s localhost singularity "$@")

local-singularity-subscribe *args='':
    (spacetime subscribe -s localhost singularity "$@")

local-singularity-log:
    (spacetime logs -s localhost -f singularity)

local-publish: local-singularity-publish
    (echo "✅ DONE.")

unsafe-local-republish: unsafe-local-singularity-drop
    (just local-publish)
