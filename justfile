prebuild:
    cargo build --release --all-features

test-oztest: prebuild
    uvx git+https://github.com/German-BioImaging/oztest.git \
        run -v parse_attributes --exclude-profile strict -- \
        ./target/release/ome_zarr_metadata_dingus

update-submodules:
    git submodule update --init --recursive --remote
