# Database

A simple database example written in Rust.

The binary can create a file with random data, and another file that indexes the `age` column.

The database can than scan the entire data through a binary search or a full search and count all the rows where the `age` column is equal to the number specified in the script.

```
$ cargo run
Full Search:
Found 102119 users with the age of 32 in 7631ms
Went through 100000 pages (pages with results 64133)

Binary Search:
Found 102119 users with the age of 32 in 5771ms
The index step took 113ms, the data step took 5658ms
Went through 64133 pages
```

The result can change depending on which `age` will be searched, or other factors (e.g. are other programs running, disk speed etc.).
