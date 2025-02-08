# Dangling Pointer in Rust

This repository demonstrates a common mistake in Rust that leads to dangling pointers. This happens when a reference or pointer to data is used after the data itself has been deallocated. In this example, we are dropping the vector before the pointer is going out of scope.