import unittest
import hello as hello

suite "description of basic test suite":
    echo "setup"

    setup:
        echo "run before each"

    teardown:
        echo "run after each"

    test "say hi":
        check(hello.say_hi() == "Hello, World!")

