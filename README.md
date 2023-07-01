<div>
    <img src="images/gitig.png">
</div>

# gitig

<img src="https://img.shields.io/github/v/release/gold24park/gitig">

Creates a `.gitignore` file for the configured project!

It is provided based on the `.gitignore` template available at [https://github.com/github/gitignore](https://github.com/github/gitignore)

## Installation

```
brew install gitig
```

## Quick Start

```
# Creates .gitignore for Android Project
gitig android
```

```
# Creates .gitignore for NodeJs Project to ~/my-node-js-project
gitig node ~/my-node-js-project
```

```
# Show available project list
gitig --list
```

```
# Show available projects starts with "no"
gitig --list no
```

## Usage

```
USAGE:
    gitig [FLAGS] [ARGS]

FLAGS:
-h, --help Prints help information
-l, --list
 -V, --version Prints version information

ARGS:
<project> The project for which to create a .gitignore [default: ]
<path> The path to create the .gitignore [default: .]

```
