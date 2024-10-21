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

## License
This program is licensed under the [GNU General Public License, version 3](LICENSE.md).

*This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public 
License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later
version.*<br />
*This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.*
<br />
*You should have received a copy of the GNU General Public License along with this program. If not, see
https://www.gnu.org/licenses/.*