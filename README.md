# The Ram programming language
A stack-based programming language developed to experiment with my language development skills. It is capable of making some mathematical formulation and logic. Written in the Rust programming language.

## How to run
1. Download the [release binary](https://github.com/ujjwal-kr/ram/releases/download/v2.7/ram).
2. Make it executable using `chmod +x ./ram`
3. Make a .ram file with the following contents at the same directory as the binary:

```as
main:
    ram lx
    ram 1
    add
    ram lx prev
    print
    ram 500
    cmp
    je halt:
    jmp main:

halt:
    halt
```

4. run `./ram ./example.ram` and press enter.

## How to test
Run `cargo run test` to run the custom tests to test all the commands in the suit. Make sure to run this before pushing any PRs. The tests are automatically ran when there is a PR/commit on the master branch.

![tests](https://cdn.discordapp.com/attachments/875733830542196768/1032414620377428059/unknown.png)

# Documentation

The documentation for using the language is available in the [wiki](https://github.com/ujjwal-kr/ram/wiki/Documentation---v2.7)

# Contributing

Im not accepting any braking changes to the specs, as the syntax of the language is still in development. But suggestions can be made using the issue tracker. All other types of contributions are welcome. Make sure to check the `dev` branch, PRs to che master branch wont be accepted.