#!/bin/bash

log_file="$HOME/file_encryption.log"

check_dependencies() {
    command -v openssl >/dev/null 2>&1 || { echo >&2 "Error: OpenSSL is not installed. Aborting."; exit 1; }
    command -v jq >/dev/null 2>&1 || { echo >&2 "Error: jq is not installed. Aborting."; exit 1; }
}

generate_iv() {
    openssl rand -hex 16
}

derive_key() {
    local password="$1"
    echo -n "$password" | openssl dgst -sha256 | awk '{print $2}'
}

log() {
    echo "$(date) [$2]: $1" >> "$log_file"
}

encrypt_data() {
    local input_file="$1"
    local password="$2"
    local create_backup="${3:-false}"

    local dir_name
    dir_name=$(dirname "$input_file")
    local base_name
    base_name=$(basename "$input_file")

    local iv key encrypted final_json

    iv=$(generate_iv)
    key=$(derive_key "$password")

    if [[ "$create_backup" == "true" ]]; then
        cp "$input_file" "$input_file.bak"
        echo "Backup created: $input_file.bak"
        log "Backup created for $input_file" "INFO"
    fi

    encrypted=$(openssl enc -aes-256-cbc -K "$key" -iv "$iv" -in "$input_file" -out /dev/stdout 2>/dev/null | base64)

    final_json=$(jq -n --arg iv "$iv" --arg data "$encrypted" '{"iv":$iv,"data":$data}')

    local tmp_file
    tmp_file=$(mktemp -t tmp)
    echo "$final_json" > "$tmp_file"

    mv "$tmp_file" "$dir_name/$base_name"
    log "File encrypted: $dir_name/$base_name" "INFO"
    echo "File encrypted and replaced: $dir_name/$base_name"
}

decrypt_data() {
    local input_file="$1"
    local password="$2"
    local create_backup="${3:-true}"

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
        log "Invalid JSON structure in $input_file" "ERROR"
        exit 2
    fi

    key=$(derive_key "$password")

    local tmp_file
    tmp_file=$(mktemp -t tmp)
    echo "$encrypted" | base64 -d | openssl enc -d -aes-256-cbc -K "$key" -iv "$iv" -out "$tmp_file" 2>/dev/null

    if [[ $? -ne 0 ]]; then
        echo "Error: Decryption failed. Please check your password and the encrypted file."
        log "Decryption failed for $input_file" "ERROR"
        exit 3
    fi

    if [[ "$create_backup" == "true" ]]; then
        mv "$input_file" "$input_file.bak"
    fi

    mv "$tmp_file" "$dir_name/$base_name"
    log "File decrypted: $dir_name/$base_name" "INFO"
    echo "File decrypted and replaced: $dir_name/$base_name"
}

process_files_in_directory() {
    local input_dir="$1"
    local password="$2"
    local operation="$3"
    local create_backup="$4"
    local file_count=0

    find "$input_dir" -type f | while read -r file; do
        echo "$operation $file..."
        if [[ "$operation" == "Encrypting" ]]; then
            encrypt_data "$file" "$password" "$create_backup"
        else
            decrypt_data "$file" "$password" "$create_backup"
        fi
        file_count=$((file_count + 1))
    done

    echo "$file_count files processed."
}

print_usage() {
    echo
    echo "Usage:"
    echo "  $0 nc, encrypt [FILE|PATH]               Encrypt a single file."
    echo "  $0 dc, decrypt [FILE|PATH]               Decrypt a single file."
    echo "  $0 ncdir, encrypt-dir [NAME|PATH]        Encrypt all files in a directory."
    echo "  $0 dcdir, decrypt-dir [NAME|PATH]        Decrypt all files in a directory."
    echo
    echo "Options:"
    echo "  --backup, -b        Create backup of original file (default: true)"
    echo "  --silent, -s        Run in silent mode (no output shown in logs)"
    echo "  --verbose, -v       Enable verbose logging for debugging"
    echo "  --help, -h          Show this help message and exit"
    echo "  --log-dir, -ld      Show the directory where the log file is stored."
    echo
    exit 0
}

check_dependencies

silent=false
create_backup=true
verbose=false

while [[ "$#" -gt 0 ]]; do
    case "$1" in
        -b|--backup)
            create_backup=true
            ;;
        -s|--silent)
            silent=true
            ;;
        -v|--verbose)
            verbose=true
            ;;
        -h|--help)
            print_usage
            ;;
        -ld|--log-dir)
            echo "Log file is stored in: $log_file"
            ;;
        *)
            break
            ;;
    esac
    shift
done

if [[ "$#" -lt 2 ]]; then
    print_usage
fi

case "$1" in
    nc|encrypt)
        operation="encrypt"
        ;;
    dc|decrypt)
        operation="decrypt"
        ;;
    ncdir|encrypt-dir)
        operation="encrypt-dir"
        ;;
    dcdir|decrypt-dir)
        operation="decrypt-dir"
        ;;
    *)
        echo "Invalid operation."
        print_usage
        exit 4
        ;;
esac

input="$2"

password=""
if [[ -z "$PASSWORD" ]]; then
    read -s -p "Enter passkey: " password
    echo
    read -s -p "Confirm passkey: " password_confirm
    echo
    if [[ "$password" != "$password_confirm" ]]; then
        echo "Passwords do not match. Aborting."
        exit 5
    fi
else
    password="$PASSWORD"
fi

case "$operation" in
    encrypt)
        encrypt_data "$input" "$password" "$create_backup"
        ;;
    decrypt)
        decrypt_data "$input" "$password" "$create_backup"
        ;;
    encrypt-dir)
        process_files_in_directory "$input" "$password" "Encrypting" "$create_backup"
        ;;
    decrypt-dir)
        process_files_in_directory "$input" "$password" "Decrypting" "$create_backup"
        ;;
esac

unset password
unset password_confirm
