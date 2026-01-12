
Memory System
=============

Normally in any other language, this is where you would be introduced to variables, data types, and syntax.
However, Dango works a bit differently.

Dango does not have variables, instead it works on a stack. The values you see that are connected to sticks
(these things, ``----``), called dumplings for lack of a better word, are pushed onto the stack in
right-to-left order.

.. code-block:: text
    
    (1)(2)(3)----

.. code-block:: text
    :caption: Stack of the example from above

    0   | 1             (top)
    1   | 2
    2   | 3             (bottom)

This means that you have to be wary of the stack whenever you are using instructions or functions that rely on them.

#####################
The ``fetch`` keyword
#####################

Since Dango uses a stack, you can only access the "top" value (the most recently pushed value). However, values lower down on the
stack can still be accessed with the ``fetch`` keyword.

``fetch`` takes in a number as input (like ``fetch 1``) and looks down that number of stack slots, finds the value there, then
duplicates it and pushes it to the top of the stack. To demonstrate, is an example:

.. code-block:: text
    :caption: looking back down the stack

    (')(Hello)----
    (')(, )----
    (')(world!)----
    eat fetch 2
    eat fetch 1
    eat fetch 0

In the code example above, ``fetch 2`` is used to get the second element from the stack top. Right before it is executed, the
stack looks like this:

.. code-block:: text
    :caption: the stack before ``fetch 2`` is executed

    0   | 'world!'      (top)
    1   | ', '
    2   | 'Hello'       (bottom)

Here, the second element from the stack top (not including the stack top itself) is the string ``Hello``. ``fetch`` takes this
value and copies it to the top of the stack, resulting in this:

.. code-block:: text
    :caption: the stack **after** ``fetch 2`` is executed

    0   'Hello'         (top)
    1   'world!'
    2   ', '
    3   | 'Hello'       (bottom)

Then, ``eat`` takes that value and pops it (removes it from the stack) and
prints it.

######################
The ``remove`` keyword
######################

If you're worried about stack overflows, then you probably shouldn't unless
you have less than 8 MB of memory. However, using up so much stack space is
very wasteful. So, the ``remove`` keyword is here to solve your problems.

The ``remove`` keyword simply pops or removes the top of the stack. For
example, in ``eat remove (1)(2)----``, the stack before ``remove`` looks like
this:

.. code-block:: text
    :caption: stack before removing top value
    
    0   | 1             (top)
    1   | 2             (bottom)

And after ``remove``, it looks like this:

.. code-block:: text
    :caption: stack after removing top value

    0   | 2             (top, bottom)

The previous top value, ``1``, is now gone without a trace!

This makes ``remove`` useful in preventing long-running programs from running
out of memory. However, it is also useful for code comments. Dango doesn't have
comments, but the ``remove`` keyword can be used to ignore any dango.

.. code-block:: text
    :caption: using ``remove (comment)----`` as a comment literal

    remove (x = input_int[])----
    (`)(:io-input)----
    remove (make sure that it is a valid number)----
    fetch 0
    (j)(1)---- eat eat (')(Invalid number. Try again.)('c)(10)(while)(=)()----
