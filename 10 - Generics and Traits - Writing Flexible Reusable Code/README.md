# Generics and Traits

Generics are a bit tricky, therefore the project inside this folder will be as simple and as to the point as possible.

## The Basics of Generics

```
fn solve<T: Float>(a: T, b: T) -> T {
	(a.powi(2) + b.powi(2)).sqrt()
}
```
**Generic Type**: Like an argument list, but for types

A generic type allows us to provide some customiation or really flexibility around not only functions, but methods, enums, vectors, structs.
<br/>
Just about everything in Rust can work with generics, and the goal of these generic types are to make your code a little bit more flexible and work with different types.

The angle brackets `<T: Float>` is a generic list.
<br/>
You can think of this as being like an argument list, but specifically for types.

