# Lessons from AoC 2021

### Day 2:
#### Rust
`std::fs::read_to_string` is a great shorthand for reading a file. Much shorted than using `BufReader` which I have been 
using.


### Day 3:
#### Rust

#### Python
Refreshed my memory on how `map` and `filter` can be implemented through list comprehensions. I could not exactly remember 
where the if condition should be in a conditional `map` vs. a `filter` comprehension.

Conditional `map`:
```python
[x if cond else y for thing in iteranle]
```
`filter`:
```python
[x for x in iterable if cond]
```
### Day 4
#### Rust

#### Python
To split a string with variable whitespace use:
```python
var.split()
```