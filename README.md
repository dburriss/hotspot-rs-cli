# hotspot-rs-cli

A CLI tool for finding hotspots in your code.

Features:

- code metrics on files
- list git contributors with metrics
- list busfactor on files
- recommend actions based on code metrics and git history

## Developer notes

- 2021/10/10: Was forced to drop down 4 versions to *rust-code-analysis* 0.0.19 as higher versions resulted in a conflict of 2 versions of *tree-sitter* being pulled in and getting a type mismatch on `get_language()`.

### Bus factor

- For each file, get authors and there additions to a file
- [ ] Simple: Count number of authors (maybe with a percentage of additions?)
- [ ] Change weighted: Count number of authors (maybe with a percentage of additions?)
- [ ] Time weighted: Weight more recent additions by not only number but also recency