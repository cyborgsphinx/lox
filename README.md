# lox
Implementations of the language described in [Crafting Interpreters](http://www.craftinginterpreters.com/)

This is split between two projects, jlox and clox. jlox is the version done in Java, and clox is done in C

I will be using checked exceptions as much as I can.
This seems to be limited to the parser unless I want to change the entire interface for the visitors to allow the interpreter to have them as well.
Since the interpreter is creating runtime exceptions anyway, it does make some sense for them to be runtime Java exceptions.

I just want more exceptions to be checked, so I'm doing that when I can.
