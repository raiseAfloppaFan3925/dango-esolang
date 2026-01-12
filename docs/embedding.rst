
Embedding Dango
===============

.. warning::

    Dango is an `esoteric programming language <https://esolangs.org/wiki/Esoteric_programming_language>`__,
    meaning that it is not intended to be used seriously. Only do this if you
    feel like it or if you want your users to suffer.

If I had added Dango to `crates.io <https://crates.io>`__, then you could just
run ``cargo add dango-esolang``. However, it's not on crates.io, so the only
way to install Dango is from source. For more detail, see :doc:`installation`.

Prerequisites
-------------

* Rust (min. edition 2024)
* Git

Embedding in a Rust crate
-------------------------

Once you have installed Dango, you can add it as a dependency to your crate.

If you installed Dango from source, then you can add this line to your
``cargo.toml``.

.. code-block:: toml

    dango = { path = "path/to/dango" }

Embedding in a C/C++ project
----------------------------

Currently, Dango has no C API and relies solely on Rust's unstable, flimsy ABI.
So I guess, make your own wrapper library and prepare for when I make the C API
exist.
