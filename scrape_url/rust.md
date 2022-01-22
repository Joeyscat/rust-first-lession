 Accepting Command Line Arguments - The Rust Programming Language  var path\_to\_root = ""; var default\_theme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "navy" : "light";  try { var theme = localStorage.getItem('mdbook-theme'); var sidebar = localStorage.getItem('mdbook-sidebar'); if (theme.startsWith('"') && theme.endsWith('"')) { localStorage.setItem('mdbook-theme', theme.slice(1, theme.length - 1)); } if (sidebar.startsWith('"') && sidebar.endsWith('"')) { localStorage.setItem('mdbook-sidebar', sidebar.slice(1, sidebar.length - 1)); } } catch (e) { }  var theme; try { theme = localStorage.getItem('mdbook-theme'); } catch(e) { } if (theme === null || theme === undefined) { theme = default\_theme; } var html = document.querySelector('html'); html.classList.remove('no-js') html.classList.remove('light') html.classList.add(theme); html.classList.add('js');  var html = document.querySelector('html'); var sidebar = 'hidden'; if (document.body.clientWidth \>= 1080) { try { sidebar = localStorage.getItem('mdbook-sidebar'); } catch(e) { } sidebar = sidebar || 'visible'; } html.classList.remove('sidebar-visible'); html.classList.add("sidebar-" + sidebar);

1. [The Rust Programming Language](title-page.html)
2. [Foreword](foreword.html)
3. [Introduction](ch00-00-introduction.html)
4. [**1.** Getting Started](ch01-00-getting-started.html)
5. 1. [**1.1.** Installation](ch01-01-installation.html)
   2. [**1.2.** Hello, World!](ch01-02-hello-world.html)
   3. [**1.3.** Hello, Cargo!](ch01-03-hello-cargo.html)

6. [**2.** Programming a Guessing Game](ch02-00-guessing-game-tutorial.html)
7. [**3.** Common Programming Concepts](ch03-00-common-programming-concepts.html)
8. 1. [**3.1.** Variables and Mutability](ch03-01-variables-and-mutability.html)
   2. [**3.2.** Data Types](ch03-02-data-types.html)
   3. [**3.3.** Functions](ch03-03-how-functions-work.html)
   4. [**3.4.** Comments](ch03-04-comments.html)
   5. [**3.5.** Control Flow](ch03-05-control-flow.html)

9. [**4.** Understanding Ownership](ch04-00-understanding-ownership.html)
10. 1. [**4.1.** What is Ownership?](ch04-01-what-is-ownership.html)
   2. [**4.2.** References and Borrowing](ch04-02-references-and-borrowing.html)
   3. [**4.3.** The Slice Type](ch04-03-slices.html)

11. [**5.** Using Structs to Structure Related Data](ch05-00-structs.html)
12. 1. [**5.1.** Defining and Instantiating Structs](ch05-01-defining-structs.html)
   2. [**5.2.** An Example Program Using Structs](ch05-02-example-structs.html)
   3. [**5.3.** Method Syntax](ch05-03-method-syntax.html)

13. [**6.** Enums and Pattern Matching](ch06-00-enums.html)
14. 1. [**6.1.** Defining an Enum](ch06-01-defining-an-enum.html)
   2. [**6.2.** The match Control Flow Operator](ch06-02-match.html)
   3. [**6.3.** Concise Control Flow with if let](ch06-03-if-let.html)

15. [**7.** Managing Growing Projects with Packages, Crates, and Modules](ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
16. 1. [**7.1.** Packages and Crates](ch07-01-packages-and-crates.html)
   2. [**7.2.** Defining Modules to Control Scope and Privacy](ch07-02-defining-modules-to-control-scope-and-privacy.html)
   3. [**7.3.** Paths for Referring to an Item in the Module Tree](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
   4. [**7.4.** Bringing Paths Into Scope with the use Keyword](ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)
   5. [**7.5.** Separating Modules into Different Files](ch07-05-separating-modules-into-different-files.html)

17. [**8.** Common Collections](ch08-00-common-collections.html)
18. 1. [**8.1.** Storing Lists of Values with Vectors](ch08-01-vectors.html)
   2. [**8.2.** Storing UTF-8 Encoded Text with Strings](ch08-02-strings.html)
   3. [**8.3.** Storing Keys with Associated Values in Hash Maps](ch08-03-hash-maps.html)

19. [**9.** Error Handling](ch09-00-error-handling.html)
20. 1. [**9.1.** Unrecoverable Errors with panic!](ch09-01-unrecoverable-errors-with-panic.html)
   2. [**9.2.** Recoverable Errors with Result](ch09-02-recoverable-errors-with-result.html)
   3. [**9.3.** To panic! or Not To panic!](ch09-03-to-panic-or-not-to-panic.html)

21. [**10.** Generic Types, Traits, and Lifetimes](ch10-00-generics.html)
22. 1. [**10.1.** Generic Data Types](ch10-01-syntax.html)
   2. [**10.2.** Traits: Defining Shared Behavior](ch10-02-traits.html)
   3. [**10.3.** Validating References with Lifetimes](ch10-03-lifetime-syntax.html)

23. [**11.** Writing Automated Tests](ch11-00-testing.html)
24. 1. [**11.1.** How to Write Tests](ch11-01-writing-tests.html)
   2. [**11.2.** Controlling How Tests Are Run](ch11-02-running-tests.html)
   3. [**11.3.** Test Organization](ch11-03-test-organization.html)

25. [**12.** An I/O Project: Building a Command Line Program](ch12-00-an-io-project.html)
26. 1. [**12.1.** Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.html)
   2. [**12.2.** Reading a File](ch12-02-reading-a-file.html)
   3. [**12.3.** Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.html)
   4. [**12.4.** Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.html)
   5. [**12.5.** Working with Environment Variables](ch12-05-working-with-environment-variables.html)
   6. [**12.6.** Writing Error Messages to Standard Error Instead of Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.html)

27. [**13.** Functional Language Features: Iterators and Closures](ch13-00-functional-features.html)
28. 1. [**13.1.** Closures: Anonymous Functions that Can Capture Their Environment](ch13-01-closures.html)
   2. [**13.2.** Processing a Series of Items with Iterators](ch13-02-iterators.html)
   3. [**13.3.** Improving Our I/O Project](ch13-03-improving-our-io-project.html)
   4. [**13.4.** Comparing Performance: Loops vs. Iterators](ch13-04-performance.html)

29. [**14.** More about Cargo and Crates.io](ch14-00-more-about-cargo.html)
30. 1. [**14.1.** Customizing Builds with Release Profiles](ch14-01-release-profiles.html)
   2. [**14.2.** Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.html)
   3. [**14.3.** Cargo Workspaces](ch14-03-cargo-workspaces.html)
   4. [**14.4.** Installing Binaries from Crates.io with cargo install](ch14-04-installing-binaries.html)
   5. [**14.5.** Extending Cargo with Custom Commands](ch14-05-extending-cargo.html)

31. [**15.** Smart Pointers](ch15-00-smart-pointers.html)
32. 1. [**15.1.** Using Box\<T\> to Point to Data on the Heap](ch15-01-box.html)
   2. [**15.2.** Treating Smart Pointers Like Regular References with the Deref Trait](ch15-02-deref.html)
   3. [**15.3.** Running Code on Cleanup with the Drop Trait](ch15-03-drop.html)
   4. [**15.4.** Rc\<T\>, the Reference Counted Smart Pointer](ch15-04-rc.html)
   5. [**15.5.** RefCell\<T\> and the Interior Mutability Pattern](ch15-05-interior-mutability.html)
   6. [**15.6.** Reference Cycles Can Leak Memory](ch15-06-reference-cycles.html)

33. [**16.** Fearless Concurrency](ch16-00-concurrency.html)
34. 1. [**16.1.** Using Threads to Run Code Simultaneously](ch16-01-threads.html)
   2. [**16.2.** Using Message Passing to Transfer Data Between Threads](ch16-02-message-passing.html)
   3. [**16.3.** Shared-State Concurrency](ch16-03-shared-state.html)
   4. [**16.4.** Extensible Concurrency with the Sync and Send Traits](ch16-04-extensible-concurrency-sync-and-send.html)

35. [**17.** Object Oriented Programming Features of Rust](ch17-00-oop.html)
36. 1. [**17.1.** Characteristics of Object-Oriented Languages](ch17-01-what-is-oo.html)
   2. [**17.2.** Using Trait Objects That Allow for Values of Different Types](ch17-02-trait-objects.html)
   3. [**17.3.** Implementing an Object-Oriented Design Pattern](ch17-03-oo-design-patterns.html)

37. [**18.** Patterns and Matching](ch18-00-patterns.html)
38. 1. [**18.1.** All the Places Patterns Can Be Used](ch18-01-all-the-places-for-patterns.html)
   2. [**18.2.** Refutability: Whether a Pattern Might Fail to Match](ch18-02-refutability.html)
   3. [**18.3.** Pattern Syntax](ch18-03-pattern-syntax.html)

39. [**19.** Advanced Features](ch19-00-advanced-features.html)
40. 1. [**19.1.** Unsafe Rust](ch19-01-unsafe-rust.html)
   2. [**19.2.** Advanced Traits](ch19-03-advanced-traits.html)
   3. [**19.3.** Advanced Types](ch19-04-advanced-types.html)
   4. [**19.4.** Advanced Functions and Closures](ch19-05-advanced-functions-and-closures.html)
   5. [**19.5.** Macros](ch19-06-macros.html)

41. [**20.** Final Project: Building a Multithreaded Web Server](ch20-00-final-project-a-web-server.html)
42. 1. [**20.1.** Building a Single-Threaded Web Server](ch20-01-single-threaded.html)
   2. [**20.2.** Turning Our Single-Threaded Server into a Multithreaded Server](ch20-02-multithreaded.html)
   3. [**20.3.** Graceful Shutdown and Cleanup](ch20-03-graceful-shutdown-and-cleanup.html)

43. [**21.** Appendix](appendix-00.html)
44. 1. [**21.1.** A - Keywords](appendix-01-keywords.html)
   2. [**21.2.** B - Operators and Symbols](appendix-02-operators.html)
   3. [**21.3.** C - Derivable Traits](appendix-03-derivable-traits.html)
   4. [**21.4.** D - Useful Development Tools](appendix-04-useful-development-tools.html)
   5. [**21.5.** E - Editions](appendix-05-editions.html)
   6. [**21.6.** F - Translations of the Book](appendix-06-translation.html)
   7. [**21.7.** G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.html)

* Light (default)
* Rust
* Coal
* Navy
* Ayu

The Rust Programming Language
==========

[](print.html) [](https://github.com/rust-lang/book)

 document.getElementById('sidebar-toggle').setAttribute('aria-expanded', sidebar === 'visible'); document.getElementById('sidebar').setAttribute('aria-hidden', sidebar !== 'visible'); Array.from(document.querySelectorAll('#sidebar a')).forEach(function(link) { link.setAttribute('tabIndex', sidebar === 'visible' ? 0 : -1); });

[Accepting Command Line Arguments](#accepting-command-line-arguments)
----------

Let’s create a new project with, as always, `cargo new`. We’ll call our project`minigrep` to distinguish it from the `grep` tool that you might already have
on your system.

```
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep

```

The first task is to make `minigrep` accept its two command line arguments: the
filename and a string to search for. That is, we want to be able to run our
program with `cargo run`, a string to search for, and a path to a file to
search in, like so:

```
$ cargo run searchstring example-filename.txt

```

Right now, the program generated by `cargo new` cannot process arguments we
give it. Some existing libraries on [crates.io](https://crates.io/) can help
with writing a program that accepts command line arguments, but because you’re
just learning this concept, let’s implement this capability ourselves.

### [Reading the Argument Values](#reading-the-argument-values) ###

To enable `minigrep` to read the values of command line arguments we pass to
it, we’ll need a function provided in Rust’s standard library, which is`std::env::args`. This function returns an iterator of the command line
arguments that were given to `minigrep`. We’ll cover iterators fully in[Chapter 13](ch13-00-functional-features.html). For now, you only need to know two details
about iterators: iterators produce a series of values, and we can call the`collect` method on an iterator to turn it into a collection, such as a vector,
containing all the elements the iterator produces.

Use the code in Listing 12-1 to allow your `minigrep` program to read any
command line arguments passed to it and then collect the values into a vector.

Filename: src/main.rs

```
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

```

Listing 12-1: Collecting the command line arguments into
a vector and printing them

First, we bring the `std::env` module into scope with a `use` statement so we
can use its `args` function. Notice that the `std::env::args` function is
nested in two levels of modules. As we discussed in [Chapter
7](ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths), in cases where the desired function is
nested in more than one module, it’s conventional to bring the parent module
into scope rather than the function. By doing so, we can easily use other
functions from `std::env`. It’s also less ambiguous than adding `use std::env::args` and then calling the function with just `args`, because `args`might easily be mistaken for a function that’s defined in the current module.

>
>
> ### [The `args` Function and Invalid Unicode](#the-args-function-and-invalid-unicode) ###
>
>
>
> Note that `std::env::args` will panic if any argument contains invalid
> Unicode. If your program needs to accept arguments containing invalid
> Unicode, use `std::env::args_os` instead. That function returns an iterator
> that produces `OsString` values instead of `String` values. We’ve chosen to
> use `std::env::args` here for simplicity, because `OsString` values differ
> per platform and are more complex to work with than `String` values.
>
>

On the first line of `main`, we call `env::args`, and we immediately use`collect` to turn the iterator into a vector containing all the values produced
by the iterator. We can use the `collect` function to create many kinds of
collections, so we explicitly annotate the type of `args` to specify that we
want a vector of strings. Although we very rarely need to annotate types in
Rust, `collect` is one function you do often need to annotate because Rust
isn’t able to infer the kind of collection you want.

Finally, we print the vector using the debug formatter, `:?`. Let’s try running
the code first with no arguments and then with two arguments:

```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/minigrep`
["target/debug/minigrep"]

```

```
$ cargo run needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
["target/debug/minigrep", "needle", "haystack"]

```

Notice that the first value in the vector is `"target/debug/minigrep"`, which
is the name of our binary. This matches the behavior of the arguments list in
C, letting programs use the name by which they were invoked in their execution.
It’s often convenient to have access to the program name in case you want to
print it in messages or change behavior of the program based on what command
line alias was used to invoke the program. But for the purposes of this
chapter, we’ll ignore it and save only the two arguments we need.

### [Saving the Argument Values in Variables](#saving-the-argument-values-in-variables) ###

Printing the value of the vector of arguments illustrated that the program is
able to access the values specified as command line arguments. Now we need to
save the values of the two arguments in variables so we can use the values
throughout the rest of the program. We do that in Listing 12-2.

Filename: src/main.rs

```
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

```

Listing 12-2: Creating variables to hold the query
argument and filename argument

As we saw when we printed the vector, the program’s name takes up the first
value in the vector at `args[0]`, so we’re starting at index `1`. The first
argument `minigrep` takes is the string we’re searching for, so we put a
reference to the first argument in the variable `query`. The second argument
will be the filename, so we put a reference to the second argument in the
variable `filename`.

We temporarily print the values of these variables to prove that the code is
working as we intend. Let’s run this program again with the arguments `test`and `sample.txt`:

```
$ cargo run test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt

```

Great, the program is working! The values of the arguments we need are being
saved into the right variables. Later we’ll add some error handling to deal
with certain potential erroneous situations, such as when the user provides no
arguments; for now, we’ll ignore that situation and work on adding file-reading
capabilities instead.

[](ch12-00-an-io-project.html) [](ch12-02-reading-a-file.html)

[](ch12-00-an-io-project.html) [](ch12-02-reading-a-file.html)

 window.playground\_copyable = true;