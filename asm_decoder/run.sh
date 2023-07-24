files=("1_single.asm" "2_multiple.asm" "3_complex_mov.asm" "4_challenge.asm")
files_encoded=("1_single" "2_multiple" "3_complex_mov" "4_challenge")
clear
echo Compile Rust
cargo build
echo Compile Asm
for str in ${files[@]}; do
    echo "Encode: ${files_encoded[$i]}"
    nasm $str
done
echo Decode Asm

for i in ${!files_encoded[@]}; do
    echo "Decode: ${files_encoded[$i]}"
    ./target/debug/asm_decoder ${files_encoded[$i]} > $((i+1))_tmp.asm
done
echo Compile decoded Asm
for i in {1..${!files[@]}; do
    echo "Encode: ${files_encoded[$i]}"
    nasm ${i}_tmp.asm
done
echo Diff
for i in ${!files_encoded[@]}; do
    diff <(xxd -b ${files_encoded[$i]}) <(xxd -b $((i+1))_tmp)
done
echo Removing tmp
for i in {1..${!files[@]}; do
    rm ${i}_tmp.asm
    rm ${i}_tmp
done
