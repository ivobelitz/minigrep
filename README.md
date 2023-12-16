# Minigrep

## Overview
A project from [The Rust Programming Language book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) which implements a number of concepts covered in the earlier chapters of the book. 

## Table of Contents
- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

[Rust + Cargo](https://doc.rust-lang.org/book/ch01-01-installation.html#installation)

## Installation

```
git clone git@github.com:ivobelitz/minigrep.git
cd minigrep
cargo build
```

## Usage
To search for a specific pattern in a file, use the following command:
```
./target/debug/minigrep pattern filename.txt
```
**Display line numbers**
```
./target/debug/minigrep -n pattern filename.txt
```

## Contributing
If you find a bug, have a feature request, or want to suggest an improvement, feel free to open a new issue and make a pull request.

## License

This project is licensed under the [MIT License](https://opensource.org/license/mit/) - see the LICENSE file for details.

The MIT License is a permissive open-source license that allows you to use, modify, and distribute the code in your own projects. You can find a copy of the license in the LICENSE file in the root of the project.

By contributing to this project, you agree that your contributions will be licensed under the MIT License.
