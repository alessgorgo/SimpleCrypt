
# 🛡️ AES Obfuscator

A Python application that allows you to obfuscate and deobfuscate text files securely using AES encryption. This tool is useful for protecting sensitive information in plain text files.

  

## 📚 Features

-  **Obfuscation**: Replace words in a text file with encrypted equivalents.

-  **Deobfuscation**: Restore the original text from the obfuscated file using a key and word mapping.

-  **Key Management**: Generate and save encryption keys securely.

-  **Word Mapping**: Keep track of the original words and their obfuscated versions.

  

## ⚙️ Requirements

- Python 3.6 or higher

-  `cryptography` library

  
  
## 📥 Installation

1.  **Clone the repository**:

```bash
git clone https://github.com/alessgorgo/aes-obfuscator.git
cd aes-obfuscator
```


2.  **Install the required libraries**:

```bash
pip install cryptography
```
  
  

## 🏗️ Compilation

The program is written in Python and can be compiled into an executable file for easier distribution. Below are the steps to create an executable file.

##### Creating an Executable with PyInstaller

**Install PyInstaller**:

```bash
pip  install  pyinstaller
```


##### Navigate  to  the  project  directory  where  ```main.py```  is  located.

1. **Create an executable**:

```bash
pyinstaller  --onefile  --windowed  main.py
```
  
2. **Find  the  executable**:
	 - After the process completes, you will find the executable in the `dist` directory created in your project folder.
  


## 🏁 Usage

#### Running  the  Application

Once  the  executable  is  created,  you  can  run  the  application  directly  without  needing  to  open  a  terminal  or  command  prompt:

###### 1. Locate  the  executable:

- For  Windows,  find  ```main.exe```  in  the  dist  folder.

- For  MacOS,  find  ```main.app```  in  the  dist  folder.


###### 2. Double-click  the  executable  to  launch  the  application.


#### How  to  Use  the  Application

###### Select  Mode:

Choose  between  _Obfuscator_  and  _Deobfuscator_  using  the  radio  buttons.


##### Obfuscation:

1. Select  the  input  text  file  you  want  to  obfuscate.

2. Choose  an  output  directory  for  the  obfuscated  file,  word  mapping  file,  and  key  file.

3. Click  on  the  Obfuscate  button.


##### Deobfuscation:

1. Select  the  obfuscated  text  file,  word  mapping  file,  and  key  file.

2. Choose  an  output  directory  for  the  restored  text  file.

3. Click  on  the  Deobfuscate  button.



#### Example

 - _Input  File_:  ```example.txt```

 - _Obfuscated  Output_:  ```obfuscated_text.txt```

 - _Word  Mapping  File_:  ```word_mapping.txt```

 - _Key  File_:  ```key.bin```

 - _Deobfuscated  Output_:  ```deobfuscated_text.txt```



#### ⚠️  Important  Notes

Ensure  to  keep  your  key  file (key.bin) secure as it is essential for deobfuscation.

The  word  mapping  file (word_mapping.txt) is necessary for restoring the original text.

  
  
## 📄 License

This  project  is  licensed  under  the  MIT  License.  See  the  **LICENSE**  file  for  details.


  
## 📞 Contact

For  any  inquiries  or  support,  feel  free  to  reach  out  to  me  at  aleksnetwork@yandex.com.
