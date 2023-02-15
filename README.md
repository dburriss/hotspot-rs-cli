# Hotspot

A CLI tool for finding hotspots in your code.

**Features:**

- [x] Code metrics on files ie LoC, Cyclomatic and cognitive complexity, 
- [x] List busfactor on files
- [x] List git contributors with metrics
- [x] List "hottest" file ie. those that have changed most frequently
- [ ] recommend actions based on code metrics and git history

## Getting

Currently only available on the releases GitHub page as binaries.

## Usage

`hotspot --help`

```
Hotspot 0.1
Devon B. <devon@chimplab.co>
Inspect source code for those hotspots based on source code metrics and change cadence

USAGE:
    hotspot.exe [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -s               Sets to silent mode
    -V, --version    Prints version information
    -v               Sets to verbose mode

OPTIONS:
    -c, --config <CONFIG_FILE>    Sets a custom config file
    -i, --include <INCLUDE>       Glob representing explicit includes
    -o, --output <REPORT_FILE>    Sets the custom output file (default is to the console)

SUBCOMMANDS:
    about           Tells more about this CLI tool
    busfactor       Calculate bus factor of repository contributors
    contributors    Gathers statistics on repository contributors
    help            Prints this message or the help of the given subcommand(s)
    hottest         Lists most changed files
    metrics         Gathers code metrics on repository
```

### Busfactor

Measures how many people need to be hit by a bus before no one who has touched that code is alive.

**Data**:  Path, Bus factor

```
USAGE:
    hotspot.exe busfactor <SOURCE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <SOURCE>    Sets the input path of source code to use [default: ./]
```

### Contributors

Lists out contributors to a git repository and some stats on the commits.

**Data**:  Contributor,  # Commits,  # Files Touched

```
USAGE:
    hotspot.exe contributors <SOURCE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <SOURCE>    Sets the input path of source code to use [default: ./]
```

### Hottest

List files in order of most changes.

**Data**: Path, # of Changes, Last changed by, Last changed at

```
USAGE:
    hotspot.exe hottest [OPTIONS] <SOURCE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --top <TOP>    Sets the number on how many results are returned. '0' returns all. [default: 0]

ARGS:
    <SOURCE>    Sets the input path of source code to use [default: ./]
```

### Metrics

Lists some basic code metrics for files in the repository.

**Data**:  File, Lines, Cognitive, Cyclomatic

```
USAGE:
    hotspot.exe metrics <SOURCE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <SOURCE>    Sets the input path of source code to use [default: ./]
```