#!/bin/bash

generate_iv() {
    openssl rand -hex 16
}

derive_key() {
    local password="$1"
    echo -n "$password" | openssl dgst -sha256 | awk '{print $2}'
}

encrypt_data() {
    local input_file="$1"
    local output_file="$2"
    local password="$3"
    local iv
    iv=$(generate_iv)
    local content
    content=$(<"$input_file")
    local key
    key=$(derive_key "$password")
    local encrypted
    encrypted=$(echo -n "$content" | openssl enc -aes-256-cbc -K "$key" -iv "$iv" -base64)
    local final_json
    final_json=$(jq -n --arg iv "$iv" --arg data "$encrypted" '{"iv":$iv,"data":$data}')
    echo "$final_json" > "$output_file"
    echo "File encrypted and saved to $output_file"
}

decrypt_data() {
    local input_file="$1"
    local output_file="$2"
    local password="$3"
    local encrypted_json
    encrypted_json=$(<"$input_file")
    echo "Raw JSON input for decryption: $encrypted_json"
    local iv
    local encrypted
    iv=$(echo "$encrypted_json" | jq -r '.iv')
    encrypted=$(echo "$encrypted_json" | jq -r '.data')
    if [[ -z "$iv" || -z "$encrypted" ]]; then
        echo "Error: Invalid JSON input. IV or encrypted data is missing."
        exit 1
    fi
    local key
    key=$(derive_key "$password")
    local decrypted
    decrypted=$(echo "$encrypted" | openssl enc -d -aes-256-cbc -K "$key" -iv "$iv" -base64)
    if [[ $? -ne 0 ]]; then
        echo "Error: Decryption failed. Please check your password and the encrypted file."
        exit 1
    fi
    echo "$decrypted" > "$output_file"
    echo "Decrypted content saved to $output_file"
}

encrypt_directory() {
    local input_dir="$1"
    local output_dir="$2"
    local password="$3"
    mkdir -p "$output_dir"
    for file in "$input_dir"/*; do
        if [[ -f "$file" ]]; then
            local filename=$(basename "$file")
            local output_file="$output_dir/$filename.json"
            encrypt_data "$file" "$output_file" "$password"
        fi
    done
}

decrypt_directory() {
    local input_dir="$1"
    local output_dir="$2"
    local password="$3"
    mkdir -p "$output_dir"
    for file in "$input_dir"/*; do
        if [[ -f "$file" ]]; then
            local filename=$(basename "$file")
            local output_file="$output_dir/$filename.dec"
            decrypt_data "$file" "$output_file" "$password"
        fi
    done
}

if [[ "$#" -ne 3 ]]; then
    echo "Usage: $0 {encrypt|decrypt|encrypt-dir|decrypt-dir} input output"
    exit 1
fi

operation="$1"
input="$2"
output="$3"

read -s -p "Enter password: " password
echo

case "$operation" in
    encrypt)
        encrypt_data "$input" "$output" "$password"
        ;;
    decrypt)
        decrypt_data "$input" "$output" "$password"
        ;;
    encrypt_dir)
        encrypt_directory "$input" "$output" "$password"
        ;;
    decrypt_dir)
        decrypt_directory "$input" "$output" "$password"
        ;;
    *)
        echo "Invalid operation. Use 'encrypt', 'decrypt', 'encrypt_dir', or 'decrypt_dir'."
        exit 1
        ;;
esac
