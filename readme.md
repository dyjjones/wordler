# Wordler

```bash
$ wordler [path-to-word-list] [regex] [comma-separated list of characters to exclude]

$ wordler ~/Documents/words.txt "..a.." "c,h,i:4,r"
```
You will need cargo to run this with either ```cargo run``` or build with ```cargo build --release``` and then copy into your path. Follow the install instructions on the Rust website and use rustup.

If you are on macos, you may have to remove a previous version of the program before re-copying it as an executable, otherwise macos may kill the process. Change paths as necessary.
```bash
$ cargo build --release
$ rm ~/Developer/bin/wordler
$ cp target/release/wordler ~/Developer/bin/wordler
```


Shows valid words that match the regular expression and which exclude the characters given. Excluded characters can have the format of a single character to exclude from the entire word, or a character followed by a colon and index to exclude words that contain that character at that index. Characters may occur multiple times as excluded-at-index instances. i.e.

```bash
$ wordler ~/Documents/words.txt ".f..r" "a:2,e:0,e:2"
```

TODO:

* Refactor cli arguments using clap.
* Order results by best choice.
* Add logic for specifying that a character occurs more than once (shows up in Wordle as two more more blocks, 1 or more green, 1 or more yellow).