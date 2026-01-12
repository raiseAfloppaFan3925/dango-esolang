
Syntax
======

In Dango, most code is written in dango.

.. code-block:: text
    :caption: A "Hello, world!" program

    eat (')(Hello, world!)----

In the code example above, you can see the word ``eat`` followed by a structure containing the actual message.
``eat`` simply removes the value at the top of the stack and prints it, but it doesn't matter right now. Let's focus
on the ``(')(Hello, world!)----'`` structure, which is called a **dango**.

A dango is made up of **dumplings** (for a lack of a better term), which contains some kind of value or command.
These dumplings must be accompanied by a stick (``----``, 4 dashes/minus signs/hyphens) to hold them together.

The
``(Hello, world!)`` dumpling is a dumpling containing **raw text**, and it pushes it to the stack. Then, the ``(')``
turns the top of the stack (which in this case, is the raw ``Hello, world!`` text) into a string. Now, the ``eat`` keyword
from earlier simply pops the top of the stack and prints it.

Code in Dango is split by lines, where lines are executed right-to-left. Keep this in mind, as it will be useful later.
