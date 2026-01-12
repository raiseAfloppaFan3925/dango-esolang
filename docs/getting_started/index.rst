
Getting Started
===============

.. note::

    This chapter assumes that you have already installed Dango. If not, then see :doc:`../installation`.

To start an interactive session, simply run dango in the command line with no arguments. This opens the interactive REPL,
which allows you to type one line of Dango code at a time. However, it only accepts one line of code.

If you installed Dango from source, then run it with ``cargo run --release``. But if you put it in your PATH, then you can
run it directly with ``dango``.

.. tab-set::

    .. tab-item:: Standalone

        .. code-block:: text

            $ dango

               _|_
              /@@@\  | Dango 0.11.0
              \@@@/  |
              /%%%\  | Documentation: https:/raiseAfloppaFan3925.github.io/dango
              \%%%/  | 'exit' to exit
              /***\  |
              \***/  | If you find any bugs, please report them at https://github.com/raiseAfloppaFan3925/dango/issues
                |    |
                |

            --(O)(O)(O) > (+)(1)(2)----
            3
            --(O)(O)(O) > eat (')(Hello, world!)----
            Hello, world!
            ()
            --(O)(O)(O) > (:math-pi)----
            3.141592653589
        
    .. tab-item:: Cargo

        .. code-block:: text

            $ cargo run --release

             _|_
            /@@@\  | Dango 0.11.0
            \@@@/  |
            /%%%\  | Documentation: https:/raiseAfloppaFan3925.github.io/dango
            \%%%/  | 'exit' to exit
            /***\  |
            \***/  | If you find any bugs, please report them at https://github.com/raiseAfloppaFan3925/dango/issues
              |    |
              |

            --(O)(O)(O) > (+)(1)(2)----
            3
            --(O)(O)(O) > eat (')(Hello, world!)----
            Hello, world!
            ()
            --(O)(O)(O) > (:math-pi)----
            3.141592653589

To run a file, put the path to it as the first argument of the ``dango`` command (or ``cargo run --release``).

.. tab-set::

    .. tab-item:: Standalone

        .. code-block:: text

            $ dango file.dango
    
    .. tab-item:: Cargo
        
        .. code-block:: text
            
            $ cargo run --release file.dango

With this, you can pass additional arguments AFTER the path to your program,
which can be used in ``(:env-args)``.

.. toctree::

    syntax
    memory
    data_types
    stdlib
