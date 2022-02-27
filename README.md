# The Ram programming language
A stack based programming language created to experiment my crappy lang-dev only capable of making some mathematical formulation and logic. Written in the rust programming language.

## How to run
1. Download the [release binary](https://github.com/ujjwal-kr/ram/releases/download/v2.1/ram).
2. Make it executable using `chmod +x ./ram`
3. Make a .ram file with the following contents at the same directory as the binary:

```as
ram rv 1
add lx rv
ram lx prev
print lx
jmp 1

ram lx
ram 10
cmp
je 2
cmp
jne 0

printc >> loop end
halt
```

4. run `./ram ./example.ram` and press enter.

## How to test
Run `cargo run test` to run the custom tests to test all the commands in the suit. Make sure to run this before pushing any PRs. The tests are automatically ran when there is a PR/commit on the master branch.

![tests](https://media.discordapp.net/attachments/867819380464680980/947347676503089172/unknown.png?width=350&height=510)

## Documentation
Official documentation for the RAM programming language. Please open an issue if you find any bugs, or want some features to be chaged or added.

## Variable Reference
The language has 6 general purpose variables (as in x86/arm) and 4 dynamic variables (as in python/js). All of them are global. The data stored in them unlike the stack is preserved when a code block is switched. Details about how to use them is further in the documentation.

### General purpose vars
- `lx` and `rv` store floating point integers.
- `string` and `lxstring` to store strings.
- `vec int` and `vec str` to store integer and string vectors.

### Dynamic vars
These variables are called hash_vars as they are implemented using a hashmap. There can be unlimited number of these named variables, restricted to types like string, integer, integer vector and string vector. You can move values from these vars to general purpose vars for further operations.

## Contents
- General commands:
  - [Comments](#comments)
  - [Print](#print)
  - [printc](#printc--characters)
  - [halt](#halt)
- std commands:
  - [stdin](#stdin)
  - [stdfs](#stdfs)
  - [random](#rand--num1num2)
- Stack based commands:
  - [ram](#ram)
  - [vectors](#vectors)
  - [split](#split)
  - [str](#str)
  - [pop](#pop)
  - [popall](#popall)
- Hash Vars
  - [var](#var)
  - [move](#move)
- Operations:  
  - [add](#add)
  - [subtract](#sub)
  - [multiply](#mul)
  - [division](#div)
  - [sqr](#sqr)
  - [sqrt](#sqrt)
  - [average](#avg)
  - [round](#round)

- [cmp and jump statements](#cmp-and-jump-statements)

## General Commands
### comments
`// this is a one line comment`

### print
- `print` prints the last number on the screen present in the stack.
- `print lx/rv` prints the value of `lx/rv` variables on the screen.
- `print string/lxstring` prints the value of `string/lxstring` variables on the screen.
- `print vec str/vec int` prints the value of `vec str/vec int` variables on the screen.
- `print var str <hash_str_var>` prints the value of `hash_str_var` variable on the screen.
- `print var str <hash_int_var>` prints the value of `hash_int_var` variable on the screen.
- `print vec str <hash_str_vec_var>` prints the value of `hash_str_vec_var` variable on the screen.
- `print vec int <hash_int_vec_var>` prints the value of `hash_int_vec_var` variable on the screen.

### printc >> characters
Prints the characters specified on the screen

### halt
`halt` stops the program

## std commands

### stdin
`stdin lx/rv/string` sets the value of the standard input provided in the next line to the specified variable.

### stdfs
- `stdfs open >> ./path/to/file` opens the file and stores the contents as strings in the  `strings` variable.

### rand >> num1,num2
`rand >> num1,num2/lx,rv/rv,lx` generates a random decimal between num1 and num2 (including both) and pushes it to stack. May `round` be used after this statement to round it to an integer.

## stack based commands

### ram
- `ram <number>` adds the number to the stack.
- `ram lx/rv <number>` assigns the number to the variable
- `ram lx/rv` adds the variable value to the stack for further calculations
- `ram lx/rv prev` assigns the last stack value to the specified variable.
- `ram string >> <strings>` assigns value to the strings global variable.
- `ram var <name>` adds the int hashvar value to the stack.
- `ram var <name> prev` assigns the last stack value to the specified hash int variable.

### vectors
- `ram vec int >> [1,2,3,4,ints...]` creates a vector and assigns it to the global `vec int` variable.
- `ram vec str >> [1,2,3,4,strs...]` creates a vector and assigns it to the global `vec str` variable.
- `vec str push` pushes the value of `string` to `vec str` variable.
- `vec int push lx/rv` pushes the value of `lx/rv` to `vec int` variable.
- `vec str >> [<index>/lx/rv]` extracts value of the index position of `vec str` and stores it in the `string` variable.
- `vec int lx/rv >> [<index>/lx/rv]` extracts the value of the index position of `vec int` and stores it in `lx/rv` variable.
- `vec str len` pushes the length of the `vec str` variable to the stack.
- `vec int len` pushes the length of the `vec int` variable to the stack.

### split
`split >> "<arg>"` splits the `string` variable into args and stores the values into the `vec str`

### str
- `str lxstring string` moves the value of the `string` variable to `lxstring`.
- `str string lxstring` moves the value of the `lxstring` variable to `string`.
- `str cmp` compares the variables `lxstring & strings` and pushes 0 to the stack if they are equal, and -1 if they are not.

### pop
Removes the last value from the stack

### popall
Empties the entire stack. Useful for `avg` implementation.

## Hash Vars

### var
- `var <name> str >> test123` creates a variable called `name` with string type and assigns "test123" to it.
- `var <name> int >> 75` creates a variable called `name` with integer type and assigns 75 to it.
- `var <name_vec> int vec >> [10, 13, 12]` creates a variable called `name_vec` with vec integer type and assigns [10, 13, 12] to it.
- `var <name_vec> str vec >> [one,two]` creates a variable called `name_vec` with vec integer type and assigns [one,two] to it.

- `var <name_vec> str vec push >> string/lxstring/var <name>` pushes string/lxstring/`<name>` into the `<name_vec>` variable.
- `var <name_vec> int vec push >> lx/rv/var <name>` pushes lx/rv/`<name>` into the `<name_vec>` variable.

- `var <name_vec> int vec lx/rv/var <name> >> [lx/rv/var name/<number>]` stores the [index] value of `<name_vec>` into the specified variable.
- `var <name_vec> str vec string/lxstring/var <name> >> [lx/rv/var name/<number>]` stores the [index] value of `<name_vec>` into the specified variable.

### move

Move is used to move the value of hashvars to general purpose vars and vice versa (note its destination first).
- `move int lx/rv <name>` moves the value of `name` hash variable to `lx/rv` (make sure `name` is of int type)
- `move str string/lxstring <name>` moves the value of `name` hash variable to `string/lxstring` (make sure `name` is of str type)
- `move str var <name> string/lxstring` moves the value of `string/lxstring` to `name` hash var (make sure `name` is of str type)
- `move int var <name> lx/rv` moves the value of `lx/rv` to `name` hash var (make sure `name` is of int type)
- `move vec vec str <name>` moves the value of `name` hash var to `vec str` (make sure `name` is a string vector)
- `move vec vec int <name>` moves the value of `name` hash var to `vec int` (make sure `name` is a integer vector)
- `move vec var <name> vec int` moves the value of `vec int` to `name` hash var (make sure `name` is a integer vector)
- `move vec var <name> vec str` moves the value of `vec str` to `name` hash var (make sure `name` is a string vector)

## Operation Commands

### add
- adds last two numbers in the stack and pushes the result to the stack
- `add lx rv` adds the two variables and pushes the result to the stack

### sub 
subtracts the last number from the number added prior to it and pushes to stack

### mul
- `mul` multiplies the last two numbers in the stack and adds the result to the stack
- `mul rv lx` multiplies lx and rv and pushes the result to the stack.

### div
the number added prior to the last number / last number and pushes to stack.

### sqr
- `sqr` squares the last number in the stack and adds the result to the stack.
- `sqr lx/rv` squares `lx/rv` and assigns the result to the variable squared.

### sqrt
- `sqrt` squares the last number in the stack and adds the result to the stack.
- `sqrt lx/rv` squares `lx/rv` and assigns the result to the variable squared.

### avg
Takes out the average of all the numbers present in the stack. `popall` should be used somewhere in the code to remove the vars which are redundant.

### round
- `round` rounds the last number present in the stack to decimals and pushes it to the stack.
- `round lx/rv` rounds the specified variable and assigns the result to the var itself.

### `cmp` and Jump statements
The program is devided into blocks separated by empty lines. And each block has an index. For example-
```as
ram 100
ram 300
cmp
jne 1
cmp
je 2

printc >> Not Equal

printc >> Equal
```

This code consists of three codeblocks, indexed 0,1,2 respectively and can be accessed by the jump statements.

- `jmp <index>` - jumps to the codeblock at the index position.
- `cmp` pushes -1, 0, 1 to the stack if the last two numbers are smaller, equal or greater than each other. (Important before jne and je commands).
- `jne <index>` - jumps to a block by its index position if the previous cmp statement is not 0 (equal)

- `je <index>` - jumps to a block by its index position if the previous cmp statement is 0 (equal)

- `jgr <index>` - jumps to a block by its index position if the prev cmp statement is 1 (greater)

- `jsm <index>` - jumps to a block by its index position if the prev cmp statement is -1 (smaller)

### Example Coin Flip

```as
printc >> Coin Flip
rand >> 0,1
round
ram 1
cmp
je 1
pop
ram 0
cmp
je 2

printc >> Heads

printc >> Tails
```
