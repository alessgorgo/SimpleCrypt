import re
import random
import string
import os
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import padding

chars = string.ascii_letters + string.digits + string.punctuation

def generate_key():
    return os.urandom(32)  # Generates a 256-bit key

def save_key(key, file_path):
    with open(file_path, 'wb') as file:
        file.write(key)

def load_key(file_path):
    with open(file_path, 'rb') as file:
        return file.read()

def encrypt_word(word, key):
    iv = os.urandom(16)  # AES block size is 16 bytes
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv), backend=default_backend())
    encryptor = cipher.encryptor()
    padder = padding.PKCS7(algorithms.AES.block_size).padder()
    padded_data = padder.update(word.encode()) + padder.finalize()
    encrypted = encryptor.update(padded_data) + encryptor.finalize()
    return iv + encrypted

def decrypt_word(encrypted_word, key):
    iv = encrypted_word[:16]
    encrypted = encrypted_word[16:]
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv), backend=default_backend())
    decryptor = cipher.decryptor()
    decrypted_padded = decryptor.update(encrypted) + decryptor.finalize()
    unpadder = padding.PKCS7(algorithms.AES.block_size).unpadder()
    decrypted = unpadder.update(decrypted_padded) + unpadder.finalize()
    return decrypted.decode()

def obfuscate_text(input_file, output_file, mapping_file, key_file):
    key = generate_key()
    save_key(key, key_file)

    with open(input_file, 'r') as file:
        text = file.read()

    word_mapping = {}
    obfuscated_text = text
    words = re.findall(r'\b\w+\b', text)
    for word in set(words):
        obfuscated_word = ''.join(random.choice(chars) for _ in range(len(word)))
        encrypted_word = encrypt_word(obfuscated_word, key)
        encrypted_word_hex = encrypted_word.hex()
        word_mapping[word] = encrypted_word_hex
        obfuscated_text = obfuscated_text.replace(word, encrypted_word_hex)

    with open(output_file, 'w') as file:
        file.write(obfuscated_text)

    with open(mapping_file, 'w') as file:
        for word, encrypted_word in word_mapping.items():
            file.write(f"{word} {encrypted_word}\n")

    return word_mapping

def deobfuscate_text(input_file, output_file, mapping_file, key_file):
    key = load_key(key_file)

    with open(input_file, 'r') as file:
        obfuscated_text = file.read()

    word_mapping = {}
    with open(mapping_file, 'r') as file:
        for line in file:
            word, encrypted_word_hex = line.split()
            word_mapping[word] = encrypted_word_hex

    for original_word, encrypted_word_hex in word_mapping.items():
        encrypted_word = bytes.fromhex(encrypted_word_hex)
        obfuscated_word = decrypt_word(encrypted_word, key)
        obfuscated_text = obfuscated_text.replace(encrypted_word_hex, original_word)

    with open(output_file, 'w') as file:
        file.write(obfuscated_text)
