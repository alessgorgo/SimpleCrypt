#!/bin/bash

config_file="$HOME/file_encryption_config.conf"
log_file="$HOME/file_encryption.log"
max_password_attempts=5

create_default_config() {
    cat > "$config_file" <<EOL
# Default configuration for file encryption

# Enable backup by default (true/false)
ENABLE_BACKUP=false

# Verbose mode to show more details (true/false)
VERBOSE=false

# Silent mode, no terminal output (true/false)
SILENT=false

# Argon2 parameters
ARGON2_TIME_COST=65536
ARGON2_MEMORY_COST=16
ARGON2_PARALLELISM=4
ARGON2_LENGTH=32
EOL
    echo "Default config file created at: $config_file"
}

if [[ ! -f "$config_file" ]]; then
    echo "Config file not found, creating default config."
    create_default_config
fi

source "$config_file"

check_dependencies() {
    local missing=()
    for cmd in openssl jq argon2; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            missing+=("$cmd")
        fi
    done
    if [[ ${#missing[@]} -ne 0 ]]; then
        echo "Error: Missing dependencies: ${missing[*]}"
        exit 1
    fi
}

get_password() {
    local password=""
    stty -echo
    read -p "Enter passkey: " password
    stty echo
    echo "$password"
}

get_file_paths() {
    local input_file="$1"
    echo "$(dirname "$input_file") $(basename "$input_file")"
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

    argon2id "$salt" -t "${ARGON2_TIME_COST:-2}" -m "${ARGON2_MEMORY_COST:-16}" -p "${ARGON2_PARALLELISM:-4}" -l "${ARGON2_LENGTH:-32}" | cut -d'$' -f6
}

trap 'rm -f "$tmp_file"; unset password; unset password_confirm' EXIT

log() {
    local message="$1"
    local level="$2"

    if [[ "$SILENT" == "true" ]]; then
        return
    fi

    echo "$(date) [$level]: $message" >> "$log_file"

    if [[ "$VERBOSE" == "true" ]]; then
        echo "$(date) [$level]: $message"
    fi
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
    local create_backup="${3:-$ENABLE_BACKUP}"

    if [[ ! -f "$input_file" || ! -r "$input_file" ]]; then
        echo "Error: Input file does not exist or is not readable."
        exit 1
    fi

    local dir_name base_name
    read dir_name base_name <<< "$(get_file_paths "$input_file")"

    local iv salt key encrypted final_json
    iv=$(openssl rand -hex 16)
    salt=$(generate_salt)

    key=$(derive_key "$password" "$salt")

    trap 'unset key' RETURN

    if [[ "$create_backup" == "true" ]]; then
        cp "$input_file" "$input_file.bak"
        echo "Backup created: $input_file.bak"
    fi

    encrypted=$(openssl enc -aes-256-cbc -K "$key" -iv "$iv" -in "$input_file" -out /dev/stdout 2>/dev/null | base64)

    if [[ -z "$encrypted" ]]; then
        echo "Error: Encryption failed, no data returned."
        exit 1
    fi

    final_json=$(jq -n --arg iv "$iv" --arg salt "$salt" --arg data "$encrypted" '{"iv":$iv,"salt":$salt,"data":$data}')

    local tmp_file
    tmp_file=$(mktemp -t tmp)

    echo "$final_json" > "$tmp_file"
    mv "$tmp_file" "$dir_name/$base_name"
    log "File encrypted and replaced: $dir_name/$base_name" "INFO"
}

decrypt_data() {
    local input_file="$1"
    local password="$2"
    local create_backup="${3:-true}"

    local dir_name base_name
    read dir_name base_name <<< "$(get_file_paths "$input_file")"

    if [[ "$base_name" == *.bak ]]; then
        echo "Skipping backup file: $input_file"
        return
    fi

    local iv encrypted salt key decrypted
    encrypted_json=$(<"$input_file")
    iv=$(echo "$encrypted_json" | jq -r '.iv')
    encrypted=$(echo "$encrypted_json" | jq -r '.data')
    salt=$(echo "$encrypted_json" | jq -r '.salt')

    if [[ -z "$iv" || -z "$encrypted" || -z "$salt" ]]; then
        echo "Error: Invalid JSON input. IV, salt, or encrypted data is missing."
        exit 2
    fi

    key=$(derive_key "$password" "$salt")

    trap 'unset key' RETURN

    local tmp_file
    tmp_file=$(mktemp "$TMPDIR/tmp.XXXXXX")

    if ! echo "$encrypted" | base64 -d | openssl enc -d -aes-256-cbc -K "$key" -iv "$iv" -out "$tmp_file" 2>/dev/null; then
        echo "Error: Decryption failed. Please check your password and the encrypted file."
        exit 3
    fi

    if [[ "$create_backup" == "true" ]]; then
        mv "$input_file" "$input_file.bak"
        echo "Backup created: $input_file.bak"
    fi

    mv "$tmp_file" "$dir_name/$base_name"
    log "File decrypted and replaced: $dir_name/$base_name" "INFO"
}

encrypt_dir() {
    local dir="$1"
    local password="$2"
    find "$dir" -type f -exec bash -c 'encrypt_data "{}" "$password" true' \;
}

decrypt_dir() {
    local dir="$1"
    local password="$2"
    find "$dir" -type f -exec bash -c 'decrypt_data "{}" "$password" true' \;
}

create_backup=${ENABLE_BACKUP:-false}

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
            echo "Log file is stored in: $config_file"
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

password=$(get_password)
check_password_complexity "$password"

if [[ "$operation" == "encrypt" ]]; then
    encrypt_data "$input" "$password" "$create_backup"
elif [[ "$operation" == "decrypt" ]]; then
    decrypt_data "$input" "$password" "$create_backup"
elif [[ "$operation" == "encrypt-dir" ]]; then
    encrypt_dir "$input" "$password"
elif [[ "$operation" == "decrypt-dir" ]]; then
    decrypt_dir "$input" "$password"
else
    echo "Unknown operation."
    exit 5
fi
