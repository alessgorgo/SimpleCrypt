#!/bin/bash

config_file="$HOME/file_encryption_config.conf"
log_file="$HOME/file_encryption.log"
public_key_path="${PUBLIC_KEY_PATH:-$HOME/public_key.pem}"
private_key_path="${PRIVATE_KEY_PATH:-$HOME/private_key.pem}"

max_password_attempts=5

create_default_config() {
    cat > "$config_file" <<EOL
# Default file encryption configuration

# Enable backup by default (true/false)
ENABLE_BACKUP=false

# Verbose mode to show more details (true/false)
VERBOSE=false

# Silent mode, no terminal output (true/false)
SILENT=false

# Key types, algorithms, and operations
KEY=RSA4K                  # Changed from "RSA 4K" to "RSA4K"
WRAP_UNWRAP=RSA-OAEP-256   # Changed from "Wrap/Unwrap" to "WRAP_UNWRAP"
SIGN_VERIFY=RS512          # Changed from "Sign/Verify" to "SIGN_VERIFY"

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

generate_rsa_keys() {
    echo "Generating RSA key pair at default locations..."
    openssl genpkey -algorithm RSA -out "$private_key_path" -pkeyopt rsa_keygen_bits:4096

    openssl rsa -pubout -in "$private_key_path" -out "$public_key_path"

    echo "Public key saved to: $public_key_path"
    echo "Private key saved to: $private_key_path"
}

if [[ ! -f "$public_key_path" ]]; then
    echo "Public key not found at $public_key_path."
    generate_rsa_keys
fi

if [[ ! -f "$private_key_path" ]]; then
    echo "Private key not found at $private_key_path."
    generate_rsa_keys
fi

key_algorithm="${KEY:-AES-256-CBC}"
wrap_algorithm="${WRAP_UNWRAP:-RSA-OAEP-256}"
sign_algorithm="${SIGN_VERIFY:-RS512}"

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

    local aes_key
    aes_key=$(openssl rand -hex 32)

    local iv
    iv=$(openssl rand -hex 16)

    local encrypted_file
    encrypted_file=$(openssl enc -aes-256-cbc -K "$aes_key" -iv "$iv" -in "$input_file" -out /dev/stdout | base64)

    if [[ -z "$encrypted_file" ]]; then
        echo "Error: Encryption failed, no data returned."
        exit 1
    fi

    local encrypted_aes_key
    encrypted_aes_key=$(echo -n "$aes_key" | openssl pkeyutl -encrypt -pubin -inkey "$public_key_path" | base64)

    local final_json
    final_json=$(jq -n --arg iv "$iv" --arg aes_key "$encrypted_aes_key" --arg data "$encrypted_file" '{
        "iv": $iv,
        "aes_key": $aes_key,
        "data": $data
    }')

    echo "$final_json" > "$input_file"
    log "File encrypted and saved as: $input_file" "INFO"
}

decrypt_data() {
    local input_file="$1"
    local password="$2"
    local create_backup="${3:-true}"

    if [[ ! -f "$input_file" || ! -r "$input_file" ]]; then
        echo "Error: Input file does not exist or is not readable."
        exit 1
    fi

    local encrypted_json
    encrypted_json=$(<"$input_file")

    echo "Debug: Encrypted JSON content:"
    echo "$encrypted_json"

    local iv encrypted_data encrypted_aes_key
    iv=$(echo "$encrypted_json" | jq -r '.iv')
    encrypted_data=$(echo "$encrypted_json" | jq -r '.data')
    encrypted_aes_key=$(echo "$encrypted_json" | jq -r '.aes_key')

    local aes_key
    aes_key=$(echo "$encrypted_aes_key" | base64 -d | openssl pkeyutl -decrypt -inkey "$private_key_path")

    local decrypted_data
    decrypted_data=$(echo "$encrypted_data" | base64 -d | openssl enc -d -aes-256-cbc -K "$aes_key" -iv "$iv")

    echo "$decrypted_data" > "$input_file"
    log "File decrypted and saved as: $input_file" "INFO"
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
    echo "  $0 nc, encrypt [FILE|PATH]                   Encrypt a single file."
    echo "  $0 dc, decrypt [FILE|PATH]                   Decrypt a single file."
    echo "  $0 ncdir, encrypt-dir [NAME|PATH]            Encrypt all files in a directory."
    echo "  $0 dcdir, decrypt-dir [NAME|PATH]            Decrypt all files in a directory."
    echo
    echo "Options:"
    echo "  --verbose, -v               Enable verbose logging for debugging"
    echo "  --help, -h                  Show this help message and exit"
    echo "  --log-dir, -ld              Show the directory where the log file is stored"
    echo "  --conf-dir, -cfd            Show the directory where the config file is stored"
    echo "  --public-key-path, -pb      Show the directory where the config file is stored"
    echo "  --private-key-path, -prv    Show the directory where the config file is stored"
    echo
    echo "Example:"
    echo "  $0 -v encrypt sample.txt"
    echo "  $0 decrypt /your/path"
    echo
    exit 0
}

if [[ $# -lt 1 ]]; then
    print_usage
fi

while [[ "$#" -gt 0 ]]; do
    case $1 in
        nc|encrypt)
            shift
            input_file="$1"
            password=$(get_password)
            encrypt_data "$input_file" "$password" "$create_backup"
            ;;
        dc|decrypt)
            shift
            input_file="$1"
            password=$(get_password)
            decrypt_data "$input_file" "$password" "$create_backup"
            ;;
        ncdir|encrypt-dir)
            shift
            dir="$1"
            password=$(get_password)
            encrypt_dir "$dir" "$password"
            ;;
        dcdir|decrypt-dir)
            shift
            dir="$1"
            password=$(get_password)
            decrypt_dir "$dir" "$password"
            ;;
        --verbose|-v)
            VERBOSE=true
            ;;
        --help|-h)
            print_usage
            ;;
        --log-dir|-ld)
            echo "Log directory: $log_file"
            exit 0
            ;;
        --conf-dir|-cfd)
            echo "Config directory: $config_file"
            exit 0
            ;;
            --public-key-path|-pb)
            echo "Public key directory $public_key_path"
            exit 0
            ;;
            --private-key-path|-prv)
            echo "Public key directory $private_key_path"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            print_usage
            ;;
    esac
    shift
done
