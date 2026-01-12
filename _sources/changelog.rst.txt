
Changelog
=========

########################
0.11.0 - 2026 January 13
########################

* Breaking changes

  * ``(len)`` now throws stack underflow error instead of returning zero if stack is empty
  
  * ``push_stack``, ``pop_stack``, and ``peek_stack`` are all replaced by ``Runtime::push``,
    ``Runtime::pop``, and ``Runtime::peek`` respectively

* Fixes
  
  * Fixed ``:io-input`` not trimming the trailing newline from the input

* Changes

  * Made the code slightly more idiomatic Rust

  * Removed the ugly ``runtime_interpret`` function and replaced it with a private method
  
  * Moved website to this ``website`` branch on the main repository
  
  * Moving the repository from ``dango-esolang`` to ``dango``

* Additions

  * New ``:chrono-now`` and ``:chrono-sleep`` functions in the standard library
  
  * New operators ``(`)`` (to integer) and ``(;)`` (to string)

  * Slightly better website
