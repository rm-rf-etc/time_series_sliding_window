defmodule SlidingWindowNifTest do
  use ExUnit.Case
  import SlidingWindowNif
  doctest SlidingWindowNif

  def ok_push({:ok, t}, row), do: push(t, row)

  test "greets the world" do
    {:ok, t} =
      new(["a", "b", "c"], 4, 5)
      |> ok_push([0.1, 0.1, 0.1])
      |> ok_push([0.2, 0.2, 0.2])
      |> ok_push([0.3, 0.3, 0.3])
      |> ok_push([0.4, 0.4, 0.4])

    assert inspect_table(t) == %{
             headers: ["a", "b", "c"],
             rows: [
               ["0.10000", "0.10000", "0.10000"],
               ["0.20000", "0.20000", "0.20000"],
               ["0.30000", "0.30000", "0.30000"],
               ["0.40000", "0.40000", "0.40000"]
             ]
           }

    {:ok, t} =
      new(["a", "b", "c"], 2, 3)
      |> ok_push([0.1, 0.1, 0.1])
      |> ok_push([0.2, 0.2, 0.2])
      |> ok_push([0.3, 0.3, 0.3])
      |> ok_push([0.4, 0.4, 0.4])

    assert inspect_table(t) == %{
             headers: ["a", "b", "c"],
             rows: [
               ["0.300", "0.300", "0.300"],
               ["0.400", "0.400", "0.400"]
             ]
           }
  end
end
