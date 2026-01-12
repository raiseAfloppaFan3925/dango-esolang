
Data Types
==========

Dango has 4 data types: **null**, **int**, **float**, **string**, and **dango**.

#########
Null type
#########

Dango has a null/void type, ``()``. This type has only one value, which is also
called ``()``.

Some standard library functions and even the "to integer" and "to float" 
operators do return this value on a failed operation, so make sure to always
check for null like it's C(++).

.. code-block:: dango
    :caption: Converting user input (a string) to a real while handling errors.

    (;)(:io-input)(0)----
    (5)(while)(!=)()---- fetch 0
    eat (Invalid number!)----
    eat (')(10)----
    (j)(1)----
    remove (Do stuff here...)----

########
Integers
########

Dango uses 64-bit signed integers. This means that the minimum value of an
integer is ``-9223372036854775808`` and the maximum value of an integer is
``9223372036854775807``.

An integer literal is a dumpling containing an integer. Unfortunately, Dango
does not have binary, hexadecimal, or octal literals.

.. code-block:: dango
    :caption: examples of integer literals (sticks removed for clarity)

    (1)
    (-314159)
    (39)
    (2763)

Because they have finite size, they will wrap around if an operation creates a
value larger than the maximum or smaller than the minimum.

.. code-block:: dango
    :caption: example of wrapping arithmetic
    
    (-)(1)(-9223372036854775808)---- remove (Subtract 1 from the minimum integer value)----
    eat (=)(9223372036854775807)---- remove (prints 1 [true])----

Dango has no boolean types, so operations that would normally return a boolean
instead return either ``0`` for false or ``1`` for true. Logical OR can be
replicated with addition, logical AND by multiplication, and logical NOT by
subtracting from one.

.. code-block:: dango
    :caption: logical AND (substitute ``a`` and ``b`` with the values you want to use for them)

    (*)(b)(a)---- remove (a * b)----

.. code-block:: dango
    :caption: logical OR (substitute ``a`` and ``b`` with the values you want to use for them)

    (+)(b)(a)---- remove (a + b)----

.. code-block:: dango
    :caption: logical NOT (substitute ``x`` with the value you want to use for it)

    (-)(x)(1)---- remove (1 - x)

######################
Floating-point numbers
######################

Floats in Dango (which will sometimes be referred to as "reals") are `64-bit
IEEE-754 double-precision floating-point numbers
<https://en.wikipedia.org/wiki/Double-precision_floating-point_format>`__,
which is ``f64`` in Rust, ``double`` in C/C++, or ``float`` in Python.

Floating-point literals are written in the same way as integer literal, with
the float in a dumpling.

.. code-block:: dango
    :caption: examples of float literals (sticks removed for clarity)

    (1.23456789)----
    (1000000000.0)----
    (0.401)----
    (-0.618)----
    (1.618)

#######
Strings
#######

Strings in Dango have no designated literal syntax unlike floats or integers,
but are instead created by stringifying a dumpling containing raw text using
the ``(')`` operator.

.. code-block:: dango
    :caption: creating a string from raw text

    (')(Hello, world!)----

Some characters (such as those reserved for commands, ``)``,  and anything
starting with a colon ``:``) cannot be created in string literals. Therefore,
the ``('c)`` operator is used. It takes in an integer as input and pushes to
the stack the character with the Unicode codepoint.

.. code-block:: dango
    :caption: printing ``(hello)``

    (')((hello)----

#####
Dango
#####

Dango (not to be confused with the code or the language itself) are arrays with
a maximum length of 5.

.. code-block:: dango
    :caption: creating a dango

    (1)(2)(3)----
    skewer 3

Dango can be created using the ``skewer`` keyword, which like ``fetch``, takes
a raw number as input. When it is executed, it pops the top N values of the
stack and puts them into a dango structure.

The only way to retrieve the elements of a dango is to pop them from it using
the ``remove`` command. Instead of just removing the dango, it removes the
head element of the dango and pushes it to the stack.

Dango can somehow contain other dango.

.. code-block:: dango
    :caption: dango nesting???

    eat skewer (1)---- skewer 2 (2)(3)---- skewer 2 (4)(5)----

.. code-block:: text
    :caption: Output

    (1)[(2)(3)][(4)(5)]----
