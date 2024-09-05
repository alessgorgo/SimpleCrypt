#!/bin/bash

CHARSET="A-Za-z0-9!@#$%^&*()_+[]{}|;:,.<>?/"
KEY_FILE="key.bin"

generate_key() {
    openssl rand -out "$KEY_FILE" 32
}

encrypt_word() {
    local word=$1
    local key=$2
    local iv
    iv=$(openssl rand -hex 16)
    encrypted=$(echo -n "$word" | openssl enc -aes-256-cbc -K "$key" -iv "$iv" -base64)
    echo "$iv$encrypted"
}

decrypt_word() {
    local encrypted_word=$1
    local key=$2
    local iv="${encrypted_word:0:32}"
    local encrypted="${encrypted_word:32}"
    decrypted=$(echo "$iv$encrypted" | openssl enc -d -aes-256-cbc -K "$key" -base64)
    echo "$decrypted"
}

obfuscate_file() {
    local file_path=$1
    local key=$(xxd -p -c 32 "$KEY_FILE")
    content=$(<"$file_path")
    obfuscated_content=$(cat /dev/urandom | tr -dc "$CHARSET" | head -c "${#content}")
    encrypted_content=$(encrypt_word "$obfuscated_content" "$key")
    echo "$encrypted_content" > "$2"
}

deobfuscate_file() {
    local file_path=$1
    local key=$(xxd -p -c 32 "$KEY_FILE")
    encrypted_content=$(<"$file_path")
    decrypted_content=$(decrypt_word "$encrypted_content" "$key")
    echo "$decrypted_content" > "$2"
}

obfuscate_directory() {
    local original_dir=$1
    local obfuscated_dir=$2
    mkdir -p "$obfuscated_dir"
    generate_key

    find "$original_dir" -type f | while read -r original_file_path; do
        relative_path=${original_file_path#$original_dir/}
        obfuscated_file_path="$obfuscated_dir/$relative_path"
        mkdir -p "$(dirname "$obfuscated_file_path")"
        obfuscate_file "$original_file_path" "$obfuscated_file_path"
        echo "Obfuscated $original_file_path -> $obfuscated_file_path"
    done
}

deobfuscate_directory() {
    local obfuscated_dir=$1
    local original_dir=$2
    mkdir -p "$original_dir"

    find "$obfuscated_dir" -type f | while read -r obfuscated_file_path; do
        relative_path=${obfuscated_file_path#$obfuscated_dir/}
        original_file_path="$original_dir/$relative_path"
        mkdir -p "$(dirname "$original_file_path")"
        deobfuscate_file "$obfuscated_file_path" "$original_file_path"
        echo "Deobfuscated $obfuscated_file_path -> $original_file_path"
    done
}

find_application_directory() {
    local app_name=$1
    local base_dir="/Applications"
    echo "Searching in base directory: $base_dir"
    local app_path="$base_dir/$app_name.app"
    if [[ -d "$app_path" ]]; then
        echo "Found application: $app_path"
        echo "$app_path"
        return
    fi

    for dir_name in "$base_dir"/*.app; do
        if [[ "${dir_name,,}" == *"$app_name"* ]]; then
            echo "Partial match found: $dir_name"
            echo "$dir_name"
            return
        fi
    done

    echo "No application found with the name: $app_name"
    echo ""
}

main() {
    echo "WARNING: The obfuscated version of the application will be created in the same directory where you run this command."
    echo "Choose an option:"
    echo "1. Obfuscate Application"
    echo "2. Deobfuscate Application"
    echo "3. Obfuscate Text File"
    echo "4. Deobfuscate Text File"
    read -p "Enter your choice (1-4): " choice

    case $choice in
        1)
            read -p "Enter the name of the application to obfuscate (without .app): " app_name
            original_dir=$(find_application_directory "$app_name")
            if [[ -n "$original_dir" ]]; then
                obfuscated_dir_name="obfuscated-$(basename "$original_dir")"
                obfuscated_dir="$(dirname "$original_dir")/$obfuscated_dir_name"
                obfuscate_directory "$original_dir" "$obfuscated_dir"
                echo "Obfuscation complete. Obfuscated files are in: $obfuscated_dir"
            else
                echo "Could not find the application directory."
            fi
            ;;
        2)
            read -p "Enter the name of the obfuscated application directory: " obfuscated_dir
            read -p "Enter the name for the original directory: " original_dir
            deobfuscate_directory "$obfuscated_dir" "$original_dir"
            echo "Deobfuscation complete. Original files are in: $original_dir"
            ;;
        3)
            read -p "Enter the text file to obfuscate: " text_file
            read -p "Enter the name for the obfuscated text file: " obfuscated_text_file
            obfuscate_file "$text_file" "$obfuscated_text_file"
            echo "Text obfuscation complete. Obfuscated file is: $obfuscated_text_file"
            ;;
        4)
            read -p "Enter the obfuscated text file to deobfuscate: " obfuscated_text_file
            read -p "Enter the name for the original text file: " original_text_file
            deobfuscate_file "$obfuscated_text_file" "$original_text_file"
            echo "Text deobfuscation complete. Original file is: $original_text_file"
            ;;
        *)
            echo "Invalid choice. Please run the script again."
            ;;
    esac
}

main
