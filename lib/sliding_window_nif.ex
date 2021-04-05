defmodule SlidingWindowNif do
  use Rustler, otp_app: :sliding_window_nif, crate: "sliding_window_nif"
  @opaque table :: reference()

  @spec new(List.t(), integer(), integer()) :: {atom(), table() | binary()}
  def new(_, _, _), do: error()

  @spec push(table(), List.t()) :: {:ok, table()} | {:error, binary()}
  def push(_, _), do: error()

  @spec replace(table(), List.t()) :: {:ok, table()} | {:error, binary()}
  def replace(_, _), do: error()

  @spec add_column(table(), binary()) :: {:ok, table()} | {:error, binary()}
  def add_column(_, _), do: error()

  @spec drop_column(table(), binary()) :: {:ok, table()} | {:error, binary()}
  def drop_column(_, _), do: error()

  @spec inspect_table(table()) :: List.t()
  def inspect_table(_), do: error()

  @spec print(table()) :: no_return()
  def print(_), do: error()

  @spec stream_csv(table(), pid(), binary()) :: :ok
  defp stream_csv(_, _, _), do: error()

  @spec csv(table(), binary(), fun() | nil) :: :done
  def csv(table, file_path, func \\ nil)
      when is_binary(file_path) and (is_function(func) or is_nil(func)) do
    case stream_csv(table, self(), file_path) do
      {:error, reason} ->
        {:error, reason}

      {:ok, _} ->
        case func do
          nil -> stream_read_loop()
          _ -> stream_read_loop(func)
        end

        {:ok, table}
    end
  end

  @spec stream_read_loop() :: no_return()
  defp stream_read_loop do
    receive do
      :done -> nil
      _ -> stream_read_loop()
    end
  end

  @spec stream_read_loop(fun()) :: no_return()
  defp stream_read_loop(func) when is_function(func) do
    receive do
      msg when msg != nil ->
        func.(msg)
        stream_read_loop(func)

      :done ->
        nil
    end
  end

  @spec stream_read_loop(nil) :: no_return()
  defp stream_read_loop(nil) do
    receive do
      msg when msg != nil -> stream_read_loop(nil)
      nil -> nil
    end
  end

  def error, do: :erlang.nif_error(:nif_not_loaded)
end
