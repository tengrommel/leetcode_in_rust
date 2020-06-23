# Storing Efficiently

you can look forward to learning about the following topics:

- Trade-offs considering speed and readability
- Accessing heap and stack variables
- How immutability influences design

# Heaps and stacks
> stack variables are preferred thanks to their low overhead and speed compared to heap-allocated data, which automatically introduces overhead thanks to the necessary heap pointer.

    use std::mem;
    
    struct MyStruct {
        a: u8,
        b: u8,
        c: u8
    }