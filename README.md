![Rust](https://github.com/reanimatorzon/tospoof/workflows/Rust/badge.svg)
# tospoof

_A binary for `hosts` file manipulations_

## Purpose

This binary helps anyone who needs to update `hosts` file frequently:
* ...dealing with the same domain name for Dev, QA, Staging, PROD environments
* ...working under migration to another content or data provider

## The way it acts

In simple words `./tospoof` works with **lists** with two or more elements.

The first argument of a list could be IPv4, IPv6 or hostname, \
All remaining elements are IP addresses.

* Build presets of your lists with YAML via `aliases.yaml`.
* Pass lists via arguments
* Combine lists

@see Usage.

## Installation
1. Run
    ```
    $ cargo +nightly build --release
    ```
2. Move `aliases.yaml` next to `./tospoof` binary
3. Update `aliases.yaml` according to your needs

## Usage
1. I believe, [`aliases.yaml`](https://github.com/reanimatorzon/tospoof/blob/master/aliases.yaml) 
   itself is a great example of how to do things

2. CLI Commands: 
  - `$ ./tospoof on <list0> <list1> ...` 
  
    Copies stdin to stdout appending its content with generated `hosts`-file entry.\
    *STDIN -> STDOUT proxying is necessary for using pipes in shell.*
    
    Generated content is the first element of the first list post-processed 
    via dig / nslookup, or a pure IPv(4|6) appended with other elements 
    of other lists preserving order.
    
  - `$ sudo ./tospoof update -v` or `# sudo ./tospoof update -v` 
  
    Writes content from STDIN to a separate block in `hosts`file.\
    Obviously, in case there is no input - the reserved block will be cleared.
    
    Manually updated data from `hosts` file won't be touched.
    By default the zone is lines between two comments: `# tospoof: {{` and `# tospoof: }}`.
    Manual changes here would make no sense after next `update` call.
    
## Examples ##

Enables preset from `aliases.yaml`:
```bash
$ ./tospoof on preset-example
127.0.0.1 example.local.dev us.example.com de.example.com de.test.net <...>
```

This example do not require any presets:
```bash
$ ./tospoof on 127.0.0.1 mydomain.com mydomain.net
127.0.0.1 mydomain.com mydomain.net
```

Combine your lists:\
(!) *Keep remember, only very first element of all the lists would be processed as a hostname*
```bash
$ ./tospoof on 'local(0, 1)' my-domains-full-list some-domains-as-one-line-array
127.0.0.1 us.example.com de.example.com de.test.net static.example.com <...>
```

Finally update our hosts file:\
(!) No preparation needed. First `update()` call 
will create working area at the end of `hosts` file.

```bash
$ ./tospoof on preset-example | ./tospoof on some-domains-as-one-line-array | sudo ./tospoof update -v
127.0.0.1 example.local.dev us.example.com de.example.com de.test.net static.example.com static.test.net
93.184.216.34 example.net
```

**Hint:** You can also play with features of YAML to build lists. 
       