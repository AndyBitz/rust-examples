# Database

A simple database example written in Rust.

The binary can create a file with random data, and another file that indexes the `age` column.

The database can than scan the entire data through a binary search or a full search and count all the rows where the `age` column is equal to the number `10`.

```
$ cargo run
Full Search:
Found 102060 users with the age of 10 in 7077 ms
Went through 100000 pages (pages with results 64242)

Binary Search:
Found 102060 users with the age of 10 in 8304 ms
Went through 64242 pages
```
