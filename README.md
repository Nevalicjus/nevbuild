# nevbuild

A small program to run scripts 'my way' without using per-lang build files.

You can create a .nb.conf file in your `$HOME` directory to overwrite the defaults provided.
If you don't, assume they are this:

```yaml
c: "gcc -o out {file} && ./out && rm out"
cpp: "g++ -o out {file} && ./out && rm out"
go: "go run {file}"
java: "javac {file} && java {name} && rm {name}.class"
js: "node {file}"
kt: "kotlinc {file} -include-runtime -d out.jar && java -jar out.jar && rm out.jar"
py: "python3 {file}"
rb: "ruby {file}"
rs: "rustc -o out {file} && ./out && rm out"
swift: "swift {file}"
ts: "node {file}"
```

`{file}` -> full name, </br>
`{name}` -> (without extension), </br>
`{ext}` -> extension

## Installation
To install nb clone this repository and run `cargo build --release`.
Afterwards, copy the built binary to your desired destination 
(most likely `sudo cp target/release/nevbuild /usr/local/bin/nb`)

## Usage
To run a file using nb, simply use:
```bash
nb <file>
```

## Shebang
nb can also be used as a [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)); simply prepend your file with:
```bash
#!/path/to/installed/nb
```
or
```bash
#!/usr/bin/env nb
```
and make it an executable (`chmod +x <file>`)
