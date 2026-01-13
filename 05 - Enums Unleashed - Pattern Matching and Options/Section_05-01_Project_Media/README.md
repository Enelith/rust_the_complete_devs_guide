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

### Adding methods to Enums
We want to add specific methods for our Enum, which will have a different behaviour depending on which kind of Enum we're dealing with.

First let's see how we would do it with Structs.
<br/>
Based on the Structs we had before, we would have 3 separate **implementations**, one for each Struct.

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

impl Book {
	fn description(&self) -> String {
		format!("Book: {} {}", self.title, self.author)
	}
}

impl Movie {
	fn description(&self) -> String {
		format!("Movie: {} {}", self.title, self.director)
	}
}

impl Audiobook {
	fn description(&self) -> String {
		format!("Audiobook: {}", self.title)
	}
}
```
While it would 100%, it's a lot of (duplicated) code.

The nice thing with Enums is that we can achieve something similar by defining just one single implementation block.

In our example, we don't know what kind of Media is `self` (Book? Movie? Audiobook?), and until we figure out, Rust won't allow us to access any properties on `self`, **even if they're coming to all 3 types** (while all 3 have a common `title` field, you would get an error such as *No field `title` in type `&Media` [E0609]* when trying to access `self.title`).

There are 2 different ways we can do it: 
```
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String }
}

impl Media {
    // 1st way: verbose, with very basic type check
    fn description_1(&self) -> String {
        if let Media::Book{ title, author } = self {
            // if we have a book
            format!("Book: {} {}", self.title, self.author)
        } else if let Media::Movie{ title, director } = self {
            // if we have a movie
            format!("Movie: {} {}", self.title, self.director)
        } else if let Media::Audiobook{ title } = self {
            // if we have an audiobook
            format!("Audiobook: {}", self.title)
        } else {
            String::format("Media description")
        }
    }

    // 2nd way: Pattern Matching statment
    fn description_2(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            _ => String::from("Media description"),
        }
    } 
}
```
- The 1st way is more verbose and uses a very basic type checking, but it's mostly use with **error handling**
- The 2nd way uses Pattern Matching statements and is usually the more favored way to handle Enums when you're trying to figure out what type the Enum is. 

## Getting Started

To run the project, ensure you have Rust installed and run:

```bash
cargo run
```
