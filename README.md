# nevbuild

A small script/bin/app to run scripts 'my way' without using per-lang build files.

Currently works for py and java.

#### py
```bash
#!/bin/bash
python3 $1
```

#### java
```bash
#!/bin/bash
#java $1
#javac $1.java && java $1 && rm $1.class
FNAME="$(awk '{split($0, array, "."); print array[1]}' <<< "$1")"
javac $FNAME.java && java $FNAME && rm $FNAME.class
```
