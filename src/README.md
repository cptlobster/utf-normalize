# dehomograph
Program for normalizing uncommon Unicode characters into their ASCII equivalents.

This was inspired by content moderation use cases where spammers will utilize unusual Unicode characters (i.e. the
Mathematical Alphanumeric Symbols group), but can likely have more use in other cases. This can assist with
accessibility, since screen readers may have trouble with these characters (and some people will use the afforementioned
group for bold/italic text on platforms that otherwise don't support it, such as Twitter).

## Running
```shell
git clone https://github.com/cptlobster/dehomograph.git
cd dehomograph
cargo run -- -i [input file]
```

### Examples
If you want to search for a word in a file using uncommon Unicode characters:
```shell
dehomograph -i message.txt | grep "beans"
```