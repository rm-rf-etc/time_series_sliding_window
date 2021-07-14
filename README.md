# SlidingWindowNif

Work in progress, not ready for use yet.

Rolling window for time-series analysis in Elixir, implemented in Rust NIF.

This NIF implements a mutable table of circular queues, and the methods for
reading and pushing to the table. From the outside, this table looks like
a fixed-length table which we can efficiently push to and read from.

```elixir
{:ok, ref} = SlidingWindowNif.new(["a", "b", "c"], 5, 5)
SlidingWindowNif.push(ref, [3.0, 3.0, 3.0])
SlidingWindowNif.push(ref, [2.0, 2.0, 2.0])
SlidingWindowNif.push(ref, [1.0, 1.0, 1.0])
SlidingWindowNif.print(ref)
```

```
mix compile
mix test
```
