The CLI Tool was aimed to alter certain texts in the .docx file and convert into pdf. The file name, the texts inside .docx file and the replaced texts all were to be provoded as arguments. Due to lack of mature and widely used library of Rust, this can not be achieved currently.

Issues faced:

1. I tried using docx crate but unfortunately there is an existing problem in the crate itself.
   T::CONTROL_BYTE,
   | ^^^^^^^^^^^^^^^ cannot perform const operation using `T`.
2. I thus tried using docx-rs which primarily is built to create a new .docx file and write contents to it. Upon using, the documents module could not be accessed as it mentions that it is a private module. I checked mod.rs file and found that it is a public module, but sadly I still can't access it.
