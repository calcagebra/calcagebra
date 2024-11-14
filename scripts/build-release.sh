cargo build --release
upx --best --lzma target/release/calcagebra
cp target/release/calcagebra .