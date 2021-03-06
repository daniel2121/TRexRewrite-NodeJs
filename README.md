# trex-rewrite-nodejs

Implement an HTTP proxy using NodeJs and Neon for TRexRewrite.

## Prerequisites

#### Server side
* [neon](https://github.com/neon-bindings/neon)
* [Rust](https://www.rust-lang.org)
* [node](https://nodejs.org)
* [sqlite]()

#### Client side
* [ANTLR4](https://github.com/antlr/antlr4/blob/master/doc/getting-started.md)
* [json-simple-1.1.jar](http://www.java2s.com/Code/JarDownload/json-simple/json-simple-1.1.jar.zip)
* jdk

## Server Installation

To build the project, execute:
```
git clone https://github.com/daniel2121/TRexRewrite-NodeJs.git

cd trex-rewrite-nodejs

neon build
```

### Detailed Installation procedure on Ubuntu/Mint:
Node Js:
```
  sudo apt-get install nodejs
  sudo ln -s `which nodejs` /usr/bin/node
  sudo apt-get install npm
```
Neon:
```
  sudo npm install -g neon-cli@0.1.7
```

Rust:
```
  curl -sf -L https://static.rust-lang.org/rustup.sh | sh
  rustup toolchain install nightly-2016-12-05
  rustup default nightly-2016-12-05
```

Sqlite:
```
sudo apt-get install libsqlite3-dev
```

TRex:
```
  git clone https://github.com/daniel2121/TRexRewrite-NodeJs.git

  cd TRexRewrite-NodeJs/

  neon build
```

NodeJs dependencies:
```
npm install express
npm install body-parser
npm install colors
```

## Run Server
Once installed and in the project folder, run:
```
node lib/index.js [testing]
```
Note: to run the test use the optional parameter "testing" (i.e. "node lib/index.js testing")

## Client Installation
```
sudo cp parser/json-simple/json-simple-1.1.jar /usr/local/lib/

cd /usr/local/lib

sudo curl -O http://www.antlr.org/download/antlr-4.5.3-complete.jar

sudo apt-get install default-jdk

sudo apt-get install jq
```
Follow instructions:

[Build Parser](https://github.com/daniel2121/TRexRewrite-NodeJs/blob/master/parser/readme.md)


## Run test
Using a different terminal, run:

```
cd scripts/tools/

./testing_fire_from_scratch.sh
```

## Tested on
```
Ubuntu 16.04 LTS
Memory: 6GB
Processor: Intel® Core™ i5-3210M CPU @ 2.50GHz × 4
OS type: 64-bit
node version: v4.2.6
rustc version: rustc 1.15.0-nightly (daf8c1dfc 2016-12-05)
neon version: 0.1.7
SQLite version: 3.11.0

openjdk version "1.8.0_131"
OpenJDK Runtime Environment (build 1.8.0_131-8u131-b11-2ubuntu1.16.04.3-b11)
OpenJDK 64-Bit Server VM (build 25.131-b11, mixed mode)
```

```
OS: Linux Mint 18.1 Cinnamon 64-bit
Version: 3.2.7

Linux Kernel: 4.4.0-53-generic
Processor: Intel Celeron CPU N3050 @ 1.60GHzx2
Memory: 1.8GiB
Hard Drives: 28.0 GB

node version: v.4.2.6
rustc version: rustc 1.16.0 (30cf806ef 2017-03-10)
neon version: 0.1.13
```

## Reference

* https://github.com/dippi/TRexRewrite

* https://github.com/deib-polimi/TRex

* https://github.com/neon-bindings/neon

* https://www.rust-lang.org
