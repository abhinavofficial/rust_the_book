# Generics - The Book
> Never to be pushed in main

## Generics
When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.

Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. **Monomorphization** is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

## Traits
A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

> Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

One restriction to note with trait implementations is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. This restriction is part of a property of programs called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

We can also specify more than one trait bound using the + syntax. The + syntax is also valid with trait bounds on generic types.

Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. So instead of writing this:
```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```
we can use a where clause, like this:
```
fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
U: Clone + Debug
{
```

This function’s signature is less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds.

By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits. 
We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait. 
Blanket implementations appear in the documentation for the trait in the “Implementors” section.

## Lifetimes
Every reference in Rust has a lifetime, which is the scope for which that reference is valid. We must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

## Documentation Strategy

| Symbol    | 	Explanation            |
|-----------|-------------------------|
| //	       | Line comment            |
| //!	      | Inner line doc comment  |
| ///	      | Outer line doc comment  |
| /*...*/	  | Block comment           |
| /*!...*/	 | Inner block doc comment |
| /**...*/	 | Outer block doc comment |