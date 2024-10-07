#!/bin/bash

check_dependencies() {
    command -v openssl >/dev/null 2>&1 || { echo >&2 "Error: OpenSSL is not installed. Aborting."; exit 1; }
    command -v jq >/dev/null 2>&1 || { echo >&2 "Error: jq is not installed. Aborting."; exit 1; }
}

generate_iv() {
    openssl rand -hex 16
}

derive_key() {
    local password="$1"
    # Use SHA-256 hash to generate a key from the password
    echo -n "$password" | openssl dgst -sha256 | awk '{print $2}'
}

encrypt_data() {
    local input_file="$1"
    local password="$2"

    # Handle full path and filename
    local dir_name
    dir_name=$(dirname "$input_file")
    local base_name
    base_name=$(basename "$input_file")

    local iv key encrypted final_json

    iv=$(generate_iv)
    key=$(derive_key "$password")

    # Encrypt the data and output to a temporary file
    encrypted=$(openssl enc -aes-256-cbc -K "$key" -iv "$iv" -in "$input_file" -out /dev/stdout | base64)

    # Store IV and encrypted data in JSON format
    final_json=$(jq -n --arg iv "$iv" --arg data "$encrypted" '{"iv":$iv,"data":$data}')

    # Save the JSON in a temporary file
    local tmp_file
    tmp_file=$(mktemp)
    echo "$final_json" > "$tmp_file"

    # Remove the original input file
    rm "$input_file"

    # Replace the original file with the encrypted content (in JSON format)
    mv "$tmp_file" "$dir_name/$base_name"
    echo "File encrypted and replaced: $dir_name/$base_name"
}

decrypt_data() {
    local input_file="$1"
    local password="$2"

    # Handle full path and filename
    local dir_name
    dir_name=$(dirname "$input_file")
    local base_name
    base_name=$(basename "$input_file")

    local iv encrypted key decrypted

    encrypted_json=$(<"$input_file")
    iv=$(echo "$encrypted_json" | jq -r '.iv')
    encrypted=$(echo "$encrypted_json" | jq -r '.data')

    if [[ -z "$iv" || -z "$encrypted" ]]; then
        echo "Error: Invalid JSON input. IV or encrypted data is missing."
        exit 2
    fi

    key=$(derive_key "$password")

    # Decrypt and store the output in a temporary file
    local tmp_file
    tmp_file=$(mktemp)
    echo "$encrypted" | base64 -d | openssl enc -d -aes-256-cbc -K "$key" -iv "$iv" -out "$tmp_file"

    if [[ $? -ne 0 ]]; then
        echo "Error: Decryption failed. Please check your password and the encrypted file."
        exit 3
    fi

    # Remove the original encrypted file
    rm "$input_file"

    # Replace the original file with the decrypted content
    mv "$tmp_file" "$dir_name/$base_name"
    echo "File decrypted and replaced: $dir_name/$base_name"
}

encrypt_directory() {
    local input_dir="$1"
    local password="$2"
    mkdir -p "$input_dir"
    for file in "$input_dir"/*; do
        if [[ -f "$file" ]]; then
            echo "Encrypting $file..."
            encrypt_data "$file" "$password"
        fi
    done
}

decrypt_directory() {
    local input_dir="$1"
    local password="$2"
    mkdir -p "$input_dir"
    for file in "$input_dir"/*; do
        if [[ -f "$file" ]]; then
            echo "Decrypting $file..."
            decrypt_data "$file" "$password"
        fi
    done
}

print_usage() {
    echo "Usage: $0 {encrypt|decrypt|encrypt-dir|decrypt-dir} input"
    echo
    echo "Options:"
    echo "  encrypt      Encrypt a single file. Original file will be replaced."
    echo "  decrypt      Decrypt a single file. Original file will be replaced."
    echo "  encrypt-dir  Encrypt all files in a directory."
    echo "  decrypt-dir  Decrypt all files in a directory."
    exit 0
}

# Main script logic
check_dependencies

if [[ "$#" -lt 2 || "$1" == "decrypt" && "$#" -ne 2 || "$1" == "encrypt-dir" && "$#" -ne 2 || "$1" == "decrypt-dir" && "$#" -ne 2 ]]; then
    print_usage
fi

operation="$1"
input="$2"

read -s -p "Enter password: " password
echo

case "$operation" in
    encrypt)
        encrypt_data "$input" "$password"
        ;;
    decrypt)
        decrypt_data "$input" "$password"
        ;;
    encrypt_dir)
        encrypt_directory "$input" "$password"
        ;;
    decrypt_dir)
        decrypt_directory "$input" "$password"
        ;;
    *)
        echo "Invalid operation. Use 'encrypt', 'decrypt', 'encrypt_dir', or 'decrypt_dir'."
        exit 4
        ;;
esac
