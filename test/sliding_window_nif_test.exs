defmodule SlidingWindowNifTest do
  use ExUnit.Case
  import SlidingWindowNif
  doctest SlidingWindowNif

  def ok_push({:ok, t}, row), do: push(t, row)
  def ok_csv({:ok, t}, file_path), do: csv(t, file_path)
  def ok_inspect_table({:ok, t}), do: inspect_table(t)

  test "create table and inspect using inspect_table/1" do
    {:ok, t} =
      new(["a", "b", "c"], 4, 5)
      |> ok_push([0.1, 0.1, 0.1])
      |> ok_push([0.2, 0.2, 0.2])
      |> ok_push([0.3, 0.3, 0.3])
      |> ok_push([0.4, 0.4, 0.4])

    assert inspect_table(t) == %{
             headers: ["a", "b", "c"],
             rows: [
               ["0.40000", "0.40000", "0.40000"],
               ["0.30000", "0.30000", "0.30000"],
               ["0.20000", "0.20000", "0.20000"],
               ["0.10000", "0.10000", "0.10000"]
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
               ["0.400", "0.400", "0.400"],
               ["0.300", "0.300", "0.300"]
             ]
           }
  end

  test "fill table from CSV and check with inspect_table/1" do
    table =
      new(["a", "b", "c"], 4, 3)
      |> ok_csv("./test.csv")
      |> ok_inspect_table()

    assert table == %{
             headers: ["a", "b", "c"],
             rows: [
               ["0.770", "0.670", "0.190"],
               ["0.610", "0.740", "0.550"],
               ["0.640", "0.260", "0.840"],
               ["0.950", "0.280", "0.560"]
             ]
           }
  end
end
