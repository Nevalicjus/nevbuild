# nevbuild

A small script/bin/app to run scripts 'my way' without using per-lang build files.

Currently works for c, cpp, elixir, java, go, js, kotlin, lua, python, ruby, rust, swift and v.

#### c
```bash
#!/bin/bash
gcc -o out $1 && ./out && rm out
```

#### cpp
```bash
#!/bin/bash
g++ $1 -o out && ./out && rm out
```

#### elixir
```bash
#!/bin/bash
elixir $1
```

#### java
```bash
#!/bin/bash
FNAME="$(awk '{split($0, array, "."); print array[1]}' <<< "$1")"
javac $FNAME.java && java $FNAME && rm $FNAME.class
```

#### go
```bash
#!/bin/bash
go run $1
```

#### js
```bash
#!/bin/bash
node $1
```

#### kotlin
```bash
#!/bin/bash
kotlinc $1 -include-runtime -d out.jar && java -jar out.jar && rm out.jar
```

#### lua
```bash
#!/bin/bash
lua $1
```

#### py
```bash
#!/bin/bash
python3 $1
```

#### ruby
```bash
#!/bin/bash
ruby $1
```

#### rust
```bash
#!/bin/bash
rustc $1 -o out && ./out && rm out
```

#### swift
```bash
#!/bin/bash
swift $1
```

#### v
```bash
#!/bin/bash
v run $1
```
