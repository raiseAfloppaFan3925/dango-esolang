
Standard Library
================

.. warning::

    This list of libraries is non-exhaustive, although there are only three libraries.

Believe it or not, Dango has a standard library. Though mostly incomplete and
very inefficient, it exists.

.. code-block:: dango
    :caption: This function has definitely popped up in earlier parts of this chapter.

    (:io-input)(0)----

##############
Function calls
##############

Before we get to the standard library, we must first discuss functions. Dango
does not have user-defined functions unlike most languages that have functions,
however it does have native functions.

The syntax for a function call is just a dumpling with the name of the function
prefixed with a colon. For example, calling a function named ``foo`` is done
with ``(:foo)``.

Since all function calls are prefixed by a colon, they can contain any
character (except ``)`` and invalid Unicode).

Function calls do not take arguments, instead they manipulate the stack
directly.

######
Chrono
######

The chrono library has only two functions, ``:chrono-now`` and
``:chrono-sleep``.

* ``:chrono-now``

  Returns the seconds since the Unix epoch in a float.

* ``:chrono-sleep``
  
  Sleeps a given number of seconds.

  * ``(:chrono-sleep)(int)----``
  * ``(:chrono-sleep)(float)----``

###
I/O
###

The I/O library has only two functions, ``:io-input`` and ``:io-write``.

* ``:io-input``

  Reads from standard input until a newline returning a string.

  * ``(:io-input)(0)----``

    Returns input from the user.

  * ``(:io-input)(1)(prompt)----``

    Prints the prompt and returns input from the user.

* ``:env-args``

  Gets the argument vector of the program. In the REPL, it is usually something
  like ``([path to dango])----``, but in a file it is ``(dango)(file)----``,
  assuming no arguments are passed.

####
Math
####

The math library provides basic trigonometric and inverse trigonometric
functions, as well as three constants.

* ``(:math-e)``

  Returns the value of :math:`e` (approximately 2.71828...)

* ``(:math-pi)``

  Returns the value of :math:`\pi` (approximately 3.14159265358979...)
