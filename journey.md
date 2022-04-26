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


## 3.15
Page 86 begun again
Tuple Structs

Block highlighting in vim
    press Esc (to leave editing or other mode)
    hit ctrl + v (visual block mode)
    use the ↑ / ↓ arrow keys to select lines you want (it won't highlight everything - it's OK!)
    Shift + i (capital I)
    insert the text you want, e.g. %
    press Esc Esc.

stopped at page 92, METHOD SYNTAX

## 3.16
Methods

Syntactically similar to functions but are called in a struct and always act on SELF

## 3.17
associated functions
functions not methods because they dont have an instance of the struct to work with

## 3.19
you can have multiple associatd functions and methods that use self for the samee stuct

Chapter 6
Enums and Pattern Matching

## 3.20
Rust does not have NULL values

Enums can be used inside structs to define a single key that haves multiple different types
On to Match
If always returns a boolean value
Match returns the matched return value

## 3.22
Repeated pattern in rust code - match against an enum, bind a variable to data inside, execute code based on it

Rust matches are exhaustive and must cover every used case or the compiler will fail - so you always need default behavior in a list of matches

Control flow with if and let

Chapter 7 - manging projects packages, crates and modules

## 3.24
Chapter 7
a package can contain multiple binary crates and optionally one library crate
packages - a cargo feature theat lets you built test and share crates
crates - trees of modules
modules and USE -
paths - naming an item (struct, function or module)

page 114 - stopping at restaurant example

## 3.26
crate
  |__ front

      |__ hosting
          |__ add_to...

module hierarchy for related functions, methods, structs, etc
This is how you build a crate with a public API and abstract the functionality into that API by using relative and absolute


privacy boundary defined with public method
requires nested PUB methods to make sure that each individual function is available into side of the module
protects the privacy of the individual methods

Making Structs and Enums PUBLIC - 120

## 3.27
Making structs public!

Getting in the weeds a bit with howw to namespace, ensure public ENUMS and STRUCTS.

## 3.30
page 125
Using External Packages
Common Collections - Chapter 8
Collections are data structures in Rust different from built-in array and tuple types. Thisey are stored on the heap because their size is unknown

Vector - Variable number of values next to each other
String - Collection of Characters
Hash Map - associate a value with a particular key, its a map

## 4.2
Vectors can only store values that are the same type

## 4.5
the `String` type
  - Growable
  - Mutable
  - Owned
  - UTF-8 Encoded String Type

`deref coercion`
page 140

## 4.6
### Strings
Indexing into Strings
  - Bytes
  - Scalar Values
  - Grapheme Clusters

  Unable to gaurantee I/O performance with a string so no indexing selecting is allowed.

### Hash Maps
`HashMap<K, V>` stores a mapping of key to type K to values of type V

## 4.16
Prolonged break!

## 4.26
Exercism.io exercises

hello world and lucians lasagna done
