defmodule SlidingWindowNifTest do
  use ExUnit.Case

  import SlidingWindowNif

  doctest SlidingWindowNif

  def ok_push({:ok, t}, row), do: push(t, row)

  test "greets the world" do
    {:ok, t} =
      new(["a", "b", "c"], 4)
      |> ok_push([0.1, 0.1, 0.1])
      |> ok_push([0.2, 0.2, 0.2])
      |> ok_push([0.3, 0.3, 0.3])
      |> ok_push([0.4, 0.4, 0.4])

    assert inspect_table(t) == %{
             headers: ["a", "b", "c"],
             rows: [
               ["0.100000", "0.100000", "0.100000"],
               ["0.200000", "0.200000", "0.200000"],
               ["0.300000", "0.300000", "0.300000"],
               ["0.400000", "0.400000", "0.400000"]
             ]
           }
  end
end
