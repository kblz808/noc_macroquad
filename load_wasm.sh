cargo build --release --target wasm32-unknown-unknown --examples

i=1
while [[ -f "target/wasm32-unknown-unknown/debug/examples/chapter${i}_*.wasm" ]]; do
    j=1
    while [[ -f "target/wasm32-unknown-unknown/debug/eamples/chapter${i}_${j}.wasm" ]]; do
        file="chapter${i}_${j}.wasm"
        cp "$file" web/wasm/
        j=$((j+1))
    done
    i=$((i+1))
done
