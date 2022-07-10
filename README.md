# memenToDo
A simple CLI app to manage reminders which
I created to get familiar with the Rust syntax.

This app has three operations:

***create [filename] [reminder]*** 

Creates a new file on disk containing *reminder*.

***show [filename]***

Shows the contents of reminder file *filename*. When filename equals 'all', a list with all reminders is printed.

***delete [filename]***

Deletes the reminder after confirmation.