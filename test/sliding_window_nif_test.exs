defmodule SlidingWindowNifTest do
  use ExUnit.Case
  doctest SlidingWindowNif

  test "greets the world" do
    assert SlidingWindowNif.hello() == :world
  end
end
