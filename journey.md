# Journey Log


https://edu.anarcho-copy.org/Programming%20Languages/Rust/rust-programming-language-steve-klabnik.pdf

## 2.16
file:///home/mupt/Downloads/rust-programming-language-steve-klabnik(1).pdf
Page 46
Rust Guessing Game

Rust variables are mutable by default

An associated function is implemented on a type, in this
case String, rather than on a particular instance of a String. Some languages
call this a static method.

Despite declaring the variable you have to write a new line so you can shadow that variable with a type other wise it sthrows a static type error
Rust is a strongly staticly typed language

Added git repo with the rust programing exercises

Successfully added anew line with the estimate for the guessing game

Listend to Podcast on changelog about rust - 45 minutes

## 2/21 Chapter 3 Begun!
https://github.com/mupt/programming_rust/tree/main/guessing_game

All variables are immutable by default
Statically typed language

Cannot assign twice to same variable

MUT shows intent to change the variable as well as making it MUTABLE

Variables vs. constants

Cannot use MUT with constants - they are always immutable

CONST keyword
Can be declared in any scope, including global scope
Cannot be set to function call or any value computed at run time

SHADOWING - reread this section on page 34
Add gitignore directories to the file!!!!

## 2.25
Page 77
Rust is an expression based language
Statements and expressions in function bodies

Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

Let x = 7;  //statement
Function definitions are also statements

Last line returned with arrow

Can return early from a function

// rust comment

INstalled rust-analyzer (https://rust-analyzer.github.io/manual.html#vimneovim)
And CoC

Real page 49!

Control FLOW


## 3.2

Wrote Fizzbuzz - used IF for control flow
Wrote fizzbuzz  - used MATCH for control flow. Love it.


Chapter 4  understanding ownership
Rust has no garbage collection

Garbage collector
Explicit allocate free memory
Memory is marnaged through a system of ownership a set of rules that the compiler checks compile time.
The stack and the heap - last in first out - data with a fixed and known size
The heap is where data with a size that may change - OS puts data in the heap and returns a pointer after allocating requested space

Each value in rust has a variable thats called its owner
Onely one owner
Owner goes out of scope, the value is dropped

## 3.5
Pg 62 - The String Type

String can be mutated, but Literals cannot (s = “”) for example

A double free error results when rust DROPS 2 or more variables that point to the same memory spot on the heap!

## 3.8
Used the Scraper crate
https://docs.rs/scraper/0.12.0/scraper/

i  think you could use this in conjunction with a web request to start actually scraping HTML very easily

References and Borrowing

## 3.10
pg70
Avoid taking ownership of a value so it doesn’t get dropped

Let mut x = 123;

&x

Both in var as parameter and argument in method signature or function express
Can’t mutate borrowed variables

Mutable references!
&mut var_name
Can only have on mutable reference in SCOPE at any given point

Prevents data races
Can’t borrow mutablea nd immutable references in the same scope

Cant return borrowed value, can return value directly

Last line of method signature is a return line

Slice type

## 3.12
Page 86 - Structs!




