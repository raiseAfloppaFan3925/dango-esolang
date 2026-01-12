
<div align="center">
<img src="https://raw.githubusercontent.com/raiseAfloppaFan3925/dango-esolang/master/assets/dango-lang-logo.svg">

<h1>Dango 0.11.0</h1>

<p>Powering applications with <a href="https://en.wikipedia.org/wiki/Dango">delicious dumplings held together by a skewer</a>.</p>

GitHub (you are here) | [Website](https://raiseafloppafan3925.github.io/dango-website/) | [Esolang Page](https://esolangs.org/wiki/Dango)
</div>

```
eat (')(Hello, world!)----
```

## Why Dango?

* **Written in [Rust](https://rust-lang.org)**: You can trust Dango to NOT crash at runtime. (does not guarantee zero panics)

* **Performance**: Dango is simple and lightweight yet fast. If you dare, you can even embed Dango into your Rust projects.

* **Extensible**: Dango is easy to extend with native functions. (but you can't have custom objects and functions aren't first-class so skill issue)

* **Active Development**: Unlike [Mango](https://github.com/raiseAfloppaFan3925/mango) and [Nonstraightforward](https://github.com/raiseAfloppaFan3925/nonstraightforward), Dango is always in development. In case it isn't, then page me on GitHub or on Esolang.

## Quick Start

See [the website](https://raiseafloppafan3925.github.io/dango-website/getting_started/index.html).

## Installing the Source

It's as simple as this.

```
git clone https://github.com/raiseAfloppaFan3925/dango-esolang.git
```

## Examples

* Humble "Hello, world!"

  ```
  eat (')(Hello, world!)----
  ```

* Avant-garde cat program

  ```
  eat (:io-input)----
  ```

* Formula for the area of a circle (`A = pi * r * r`)

  ```
  remove (The value below is the radius. Replace it with whatever expression you want.)----
  (1)----

  (*)(:math-pi)(*)---- fetch 0
  ```

> **Note**: raiseAfloppaFan3925 does NOT guarantee the [Turing-completeness](https://en.wikipedia.org/wiki/Turing_completeness) of Dango. Although, I do assume that it's somewhere around a [linear-bounded automaton](https://esolangs.org/wiki/LBA).

> **Serious Note**: Dango is an [esolang](https://esolangs.org/wiki/Esoteric_programming_language), so I don't actually expect that you migrate your projects from C/C++/Rust/Python/Ruby/Swift/Scala/JavaScript/TypeScript/Yappacino/Nonstraightforward/WhateverLanguagesYouAreUsing to Dango.
