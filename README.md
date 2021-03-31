# SlidingWindowNif

Work in progress, not ready for use yet.

Rolling window for time-series analysis in Elixir, implemented
in Rust NIF.

```elixir
{:ok, ref} = SlidingWindowNif.new(["a", "b", "c"], 5)
SlidingWindowNif.push(ref, [3.0, 3.0, 3.0])
SlidingWindowNif.push(ref, [2.0, 2.0, 3.0])
SlidingWindowNif.push(ref, [1.0, 1.0, 3.0])
SlidingWindowNif.print(ref)
```
