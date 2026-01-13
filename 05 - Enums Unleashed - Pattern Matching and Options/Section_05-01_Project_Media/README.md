# Section_05-01_Project_Media

## Introduction

This application is going to represent a Media Storage App.

In our app, we're going to create Books (which have title and author as String), Movies (which have title and director as String), 
and Audiobook (which will only have title as String).

We're also going to create some sort of Catalog: it will be used as some kind of central repository for all our media.

Once we've created our Catalog, we should be able to add Books, Movies, and Audiobooks to it.

Once all sort kind of medias are stored into it, we want to have some sort of basic operations, such as searching for Books/Movies/Audiobooks (based on the title, the author, director, and so on). 

## How to start

To get started on our project, the first thing we should do is to figure some way to represent Books, Movies, and Audiobooks inside our code.

One way we could do this is by making 3 different kind of Structs.

```
struct Book {
	title: String,
	author: String,
}

struct Movie {
	title: String,
	director: String,
}

struct Audiobook {
	title: String,
}
```
This would absolutely work for this program.

However, there's something to point out here: by having a look at Book, Movie, and Audiobook, you quickly realize that they're all very similar in nature. 
They all have titles, but they're still all slightly/distinctly different (a Book has an author, a Movie has a director, and an Audiobook has neither).
<br/>
*We need to model several different things that are all kind of similar*.

Whenever we have datas like this inside of Rust, we therefore have 2 options for representing it:
- We can either use Structs has shown above
- Or we could use an Enumeration/Enum (note that Enums in Rust are a little different than enums in other languages ~ they have a lot of extra functionalities as well, which makes them very suitable to represent those kind of similar yet distinct datas)

**Example of Enum:**
```
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String }
}
```
- We can **imagine** that this creates three structs
- Book, Movie and Audiobook are all of type `Media`
- We can define functions that accept values of type `Media`, and put in a Book, a Movie, or an Audiobook

## Getting Started

To run the project, ensure you have Rust installed and run:

```bash
cargo run
```
