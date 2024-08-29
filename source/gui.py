import tkinter as tk
from tkinter import filedialog, messagebox
import os
from backend import obfuscate_text, deobfuscate_text

def select_input_file():
    file_path = filedialog.askopenfilename()
    if file_path:
        input_entry.delete(0, tk.END)
        input_entry.insert(0, file_path)

def select_output_directory():
    directory = filedialog.askdirectory()
    if directory:
        output_entry.delete(0, tk.END)
        output_entry.insert(0, directory)

def obfuscate():
    input_file = input_entry.get()
    output_directory = output_entry.get()
    if not input_file or not output_directory:
        messagebox.showwarning("Input Required", "Please select an input file and an output directory.")
        return

    output_file = os.path.join(output_directory, "obfuscated_text.txt")
    mapping_file = os.path.join(output_directory, "word_mapping.txt")
    key_file = os.path.join(output_directory, "key.bin")
    
    try:
        obfuscate_text(input_file, output_file, mapping_file, key_file)
        messagebox.showinfo("Success", f"File obfuscated successfully.\nWord mapping saved as: {mapping_file}\nKey saved as: {key_file}")
    except Exception as e:
        messagebox.showerror("Error", f"An error occurred: {e}")
        return

def deobfuscate():
    input_file = input_entry.get()
    output_directory = output_entry.get()
    mapping_file = mapping_entry.get()
    key_file = key_entry.get()
    if not input_file or not output_directory or not mapping_file or not key_file:
        messagebox.showwarning("Input Required", "Please select an input file, a mapping file, a key file, and an output directory.")
        return

    output_file = os.path.join(output_directory, "deobfuscated_text.txt")
    
    try:
        deobfuscate_text(input_file, output_file, mapping_file, key_file)
        messagebox.showinfo("Success", "File de-obfuscated successfully.")
    except Exception as e:
        messagebox.showerror("Error", f"An error occurred: {e}")
        return

def toggle_mode():
    mode = mode_var.get()
    if mode == "obfuscate":
        mapping_label.grid_remove()
        mapping_entry.grid_remove()
        mapping_button.grid_remove()
        key_label.grid_remove()
        key_entry.grid_remove()
        key_button.grid_remove()
        obfuscate_button.grid()
        deobfuscate_button.grid_remove()
    else:
        mapping_label.grid()
        mapping_entry.grid()
        mapping_button.grid()
        key_label.grid()
        key_entry.grid()
        key_button.grid()
        obfuscate_button.grid_remove()
        deobfuscate_button.grid()

# GUI Setup
root = tk.Tk()
root.title("File De&Obfuscator")

mode_var = tk.StringVar(value="obfuscate")

frame = tk.Frame(root, padx=10, pady=10)
frame.pack(padx=10, pady=10)

mode_label = tk.Label(frame, text="Select Mode:")
mode_label.grid(row=0, column=0, sticky="e")

obfuscate_radio = tk.Radiobutton(frame, text="Obfuscator", variable=mode_var, value="obfuscate", command=toggle_mode)
obfuscate_radio.grid(row=0, column=1, sticky="w")

deobfuscate_radio = tk.Radiobutton(frame, text="Deobfuscator", variable=mode_var, value="deobfuscate", command=toggle_mode)
deobfuscate_radio.grid(row=0, column=2, sticky="w")

input_label = tk.Label(frame, text="Select Input File:")
input_label.grid(row=1, column=0, sticky="e")

input_entry = tk.Entry(frame, width=50)
input_entry.grid(row=1, column=1, padx=5, pady=5)

input_button = tk.Button(frame, text="Browse...", command=select_input_file)
input_button.grid(row=1, column=2, padx=5, pady=5)

output_label = tk.Label(frame, text="Select Output Directory:")
output_label.grid(row=2, column=0, sticky="e")

output_entry = tk.Entry(frame, width=50)
output_entry.grid(row=2, column=1, padx=5, pady=5)

output_button = tk.Button(frame, text="Browse...", command=select_output_directory)
output_button.grid(row=2, column=2, padx=5, pady=5)

mapping_label = tk.Label(frame, text="Select Word Mapping File:")
mapping_label.grid(row=3, column=0, sticky="e")
mapping_label.grid_remove()

mapping_entry = tk.Entry(frame, width=50)
mapping_entry.grid(row=3, column=1, padx=5, pady=5)
mapping_entry.grid_remove()

mapping_button = tk.Button(frame, text="Browse...", command=lambda: mapping_entry.insert(0, filedialog.askopenfilename()))
mapping_button.grid(row=3, column=2, padx=5, pady=5)
mapping_button.grid_remove()

key_label = tk.Label(frame, text="Select Key File:")
key_label.grid(row=4, column=0, sticky="e")
key_label.grid_remove()

key_entry = tk.Entry(frame, width=50)
key_entry.grid(row=4, column=1, padx=5, pady=5)
key_entry.grid_remove()

key_button = tk.Button(frame, text="Browse...", command=lambda: key_entry.insert(0, filedialog.askopenfilename()))
key_button.grid(row=4, column=2, padx=5, pady=5)
key_button.grid_remove()

obfuscate_button = tk.Button(frame, text="Obfuscate", command=obfuscate)
obfuscate_button.grid(row=5, column=1, pady=10)

deobfuscate_button = tk.Button(frame, text="Deobfuscate", command=deobfuscate)
deobfuscate_button.grid(row=5, column=1, pady=10)
deobfuscate_button.grid_remove()

root.mainloop()
