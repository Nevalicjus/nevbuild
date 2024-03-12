# nevbuild

A small script/bin/app to run scripts 'my way' without using per-lang build files.

You can create a .nb.conf file in your $HOME directory to overwrite the defaults provided.
If you don't, assume they are this:

```yaml
c: "gcc -o out {file} && ./out && rm out"
cpp: "g++ -o out {file} && ./out && rm out"
exs: "elixir {file}"
go: "go run {file}"
java: "javac {file} && java {name} && rm {name}.class"
js: "node {file}"
kt: "kotlinc {file} -include-runtime -d out.jar && java -jar out.jar && rm out.jar"
lua: "lua {file}"
py: "python3 {file}"
rb: "ruby {file}"
rs: "rustc -o out {file} && ./out && rm out"
swift: "swift {file}"
ts: "node {file}"
v: "v run {file}"
```

`{file}` -> full name, </br>
`{name}` -> (without extension), </br>
`{ext}` -> extension
