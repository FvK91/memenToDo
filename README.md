# memenToDo
A simple CLI app to manage reminders which
I created to get familiar with the Rust syntax. The name of this app is derived from the movie: [Memento (2000)](https://www.imdb.com/title/tt0209144/)

This app has three operations:

---

- ***create [filename] [reminder]*** 

    Creates a new file on disk containing *reminder*.
---
- ***show [filename]***

    Shows the contents of *filename*. When filename equals 'all', a list with all reminders is printed.

---

- ***delete [filename]***

    Deletes the *filename* after confirmation.