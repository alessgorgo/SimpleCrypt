#!/bin/bash

config_file="$HOME/file_encryption_config.conf"
log_file="$HOME/file_encryption.log"
max_password_attempts=5

if [[ -f "$config_file" ]]; then
    source "$config_file"
fi

check_dependencies() {
    for cmd in openssl jq argon2; do
        command -v "$cmd" >/dev/null 2>&1 || { echo >&2 "Error: $cmd is not installed. Aborting."; exit 1; }
    done
}

generate_iv() {
    openssl rand -hex 12
}

generate_salt() {
    openssl rand -hex 16
}

derive_key() {
    local password="$1"
    local salt="$2"

    argon2id "$salt" "$password" -t 65536 -m 16 -p 4 -l 32 | cut -d'$' -f3
}

log() {
    echo "$(date) [$2]: $1" >> "$log_file"
}

check_password_complexity() {
    local password="$1"
    if [[ ${#password} -lt 8 || ! "$password" =~ [A-Z] || ! "$password" =~ [a-z] || ! "$password" =~ [0-9] || ! "$password" =~ [^a-zA-Z0-9] ]]; then
        echo "Error: Password must be at least 8 characters long and contain uppercase, lowercase, digits, and special characters."
        exit 6
    fi
}

encrypt_data() {
    local input_file="$1"
    local password="$2"
    local create_backup="${3:-false}"

    if [[ ! -f "$input_file" || ! -r "$input_file" ]]; then
        echo "Error: Input file does not exist or is not readable."
        exit 1
    fi

    local dir_name
    dir_name=$(dirname "$input_file")
    local base_name
    base_name=$(basename "$input_file")

    local iv salt key encrypted final_json
    iv=$(openssl rand -hex 16)
    salt=$(generate_salt)

    key=$(derive_key "$password" "$salt")

    echo "Debug Info: Key Length=${#key}, IV Length=${#iv}"

    if [[ "$create_backup" == "true" ]]; then
        cp "$input_file" "$input_file.bak"
        echo "Backup created: $input_file.bak"
    fi

    echo "Debug Info: Input File='$input_file', Key='$key', IV='$iv'"

    encrypted=$(openssl enc -aes-256-cbc -K "$key" -iv "$iv" -in "$input_file" -out /dev/stdout 2>/dev/null | base64)

    if [[ -z "$encrypted" ]]; then
        echo "Error: Encryption failed, no data returned."
        echo "Debug Info: Key='$key', IV='$iv'"
        exit 1
    fi

    echo "Debug Info: Encrypted Data (Base64)='$encrypted'"

    final_json=$(jq -n --arg iv "$iv" --arg salt "$salt" --arg data "$encrypted" '{"iv":$iv,"salt":$salt,"data":$data}')

    local tmp_file
    tmp_file=$(mktemp -t tmp)
    echo "$final_json" > "$tmp_file"

    mv "$tmp_file" "$dir_name/$base_name"
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

    if [[ "$base_name" == *.bak ]]; then
        echo "Skipping backup file: $input_file"
        return
    fi

    local iv encrypted salt key decrypted
    encrypted_json=$(<"$input_file")
    iv=$(echo "$encrypted_json" | jq -r '.iv')
    encrypted=$(echo "$encrypted_json" | jq -r '.data')
    salt=$(echo "$encrypted_json" | jq -r '.salt')

    echo "Debug Info: IV=$iv, Salt=$salt, Encrypted Data=$encrypted"

    if [[ -z "$iv" || -z "$encrypted" || -z "$salt" ]]; then
        echo "Error: Invalid JSON input. IV, salt, or encrypted data is missing."
        exit 2
    fi

    key=$(derive_key "$password" "$salt")

    local tmp_file
    tmp_file=$(mktemp "$TMPDIR/tmp.XXXXXX")

    echo "$encrypted" | base64 -d | openssl enc -d -aes-256-cbc -K "$key" -iv "$iv" -out "$tmp_file" 2>/dev/null

    if [[ $? -ne 0 ]]; then
        echo "Error: Decryption failed. Please check your password and the encrypted file."
        exit 3
    fi

    if [[ "$create_backup" == "true" ]]; then
        mv "$input_file" "$input_file.bak"
        echo "Backup created: $input_file.bak"
    fi

    mv "$tmp_file" "$dir_name/$base_name"
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
    echo "  --backup, -b        Create backup of original file in the same folder (default: false)"
    echo "  --silent, -s        Run in silent mode (no output shown in logs)"
    echo "  --verbose, -v       Enable verbose logging for debugging"
    echo "  --help, -h          Show this help message and exit"
    echo "  --log-dir, -ld      Show the directory where the log file is stored"
    echo "  --conf-dir, -cfd    Show the directory where the config file is stored"
    echo
    echo "Example:"
    echo "  $0 -b nc sample.txt"
    echo "  $0 -s nc /your/path"
    echo
    exit 0
}

check_dependencies

silent=false
create_backup=false
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
            exit 0
            ;;
        -cfd|--conf-dir)
            #echo "Config file is stored in: $config_file"
            echo "Config not implemented yet."
            exit 0
            ;;
        *)
            break
            ;;
    esac
done

if [[ "$#" -lt 2 ]]; then
    echo "Error: Missing required arguments."
    print_usage
fi

case "$1" in
    nc|encrypt)
        operation="encrypt"
        input="$2"
        ;;
    dc|decrypt)
        operation="decrypt"
        input="$2"
        ;;
    ncdir|encrypt-dir)
        operation="encrypt-dir"
        input="$2"
        ;;
    dcdir|decrypt-dir)
        operation="decrypt-dir"
        input="$2"
        ;;
    *)
        echo "Invalid operation."
        print_usage
        exit 4
        ;;
esac

password_attempts=0
password=""
while [[ $password_attempts -lt $max_password_attempts ]]; do
    read -s -p "Enter passkey: " password
    echo
    check_password_complexity "$password"
    read -s -p "Confirm passkey: " password_confirm
    echo
    if [[ "$password" == "$password_confirm" ]]; then
        break
    else
        echo "Passwords do not match. Please try again."
        ((password_attempts++))
        if [[ $password_attempts -eq $max_password_attempts ]]; then
            echo "Error: Maximum password attempts exceeded. Aborting."
            exit 5
        fi
    fi
done

case "$operation" in
    encrypt)
        encrypt_data "$input" "$password" "$create_backup"
        ;;
    decrypt)
        decrypt_data "$input" "$password" "$create_backup"
        ;;
    encrypt-dir)
        echo "Encrypting directory not implemented yet."
        exit 7
        ;;
    decrypt-dir)
        echo "Decrypting directory not implemented yet."
        exit 7
        ;;
esac

unset password
unset password_confirm
