# Iterators

- Used to iterate over any kind of data structure
- We've already been using them ~ they are used behind the scenes when you write a `for loop`
- Follow all the same rules of **Ownership**, **Borrowing**, and **Lifetimes**
- Use the `Option` enum 

## Basics of Iterator

In order to create an iterator, you need to use the method `.iter()` on a data structure (an example would be a `Vec<String>`, but it can also be used on other data structures).

Suppose we got a vector of strings,
```
let colors = vec![
	String::from("red"),
	String::from("green"),
	String::from("blue"),
]
``` 
When we call that `.iter()` method, we are creating a brand new iterator.
<br/>
This iterator is a struct. It has its own set of fields, its own data, and its own functions as well.

```
Iter<String> {
	pointer_to_data: ..., // => it's pointing back to our vector
	pointer_to_current_position: ..., // => usually a reference to the first element inside this data structure ("red")
	pointer_to_end: ... // => is a pointer, just outside to kind of the last space OUTSIDE of this entire vector 
}
```
One of the most important thing to understand about Iterators, right from the get go, is that they are completely separate from the data structures that we are iterating over.

So when we make an iterator, this is a brand new and completely separate thing from our vector. They are 2 separate objects.

The iterator is just a tool that we have to iterate through all the different elements inside the vector (`red`, `green`, and `blue`). 

- `.next()`: the `.next()` method is how we iterate through this iterator and kind of walk through all the different elements inside our vector.
	- So when we call `.next()`, behind the scenes, the iterator is going to take a look at whatever the `pointer_to_current_position` is looking at (`red`).
	- It's going to get wrapped up with a `Some()` (`Some(red)`)coming from the `Option` enum, that's we get back when we call `.next()`
	- As soon as we call `.next()`, we're also going to automatically move the `pointer_to_current_position` pointer down to the next element (`green`)
	- After calling `.next()` a 4th time, `pointer_to_current_position` will be pointing down at the very kind of OUTSIDE EDGE of our vector: that's a sign we have nothing else to iterator over
		- So we know we have nothing to iterate over when the `pointer_to_current_position` pointer is equal to the `pointer_to_end` pointer.
	- In that last scenario, when we're calling `.next()`, we're going to get back `None` to indicate that's it, we're all done with iteration.

In short, having a `Some` is a sign we can still iterate over, while having a `None` means there's nothing to iterate over anymore.

Because the `pointer_to_current_position` reference keeps changing during this process, when defining an iterator, the `mut` keyword is required.
```
let mut colors_iter = colors.iter();
```

Reminder: We make use of the `mut` keyword anytime we expact to reassign a variable, or we expect to modify the value in some way.
