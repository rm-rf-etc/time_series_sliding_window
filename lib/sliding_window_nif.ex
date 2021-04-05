defmodule SlidingWindowNif do
  use Rustler, otp_app: :sliding_window_nif, crate: "sliding_window_nif"
  @opaque window :: reference()

  @spec new(List.t(), integer(), integer()) :: {atom(), window() | binary()}
  def new(_, _, _), do: error()

  @spec push(window(), List.t()) :: {:ok, window()} | {:error, binary()}
  def push(_, _), do: error()

  @spec replace(window(), List.t()) :: {:ok, window()} | {:error, binary()}
  def replace(_, _), do: error()

  @spec add_column(window(), binary()) :: {:ok, window()} | {:error, binary()}
  def add_column(_, _), do: error()

  @spec drop_column(window(), binary()) :: {:ok, window()} | {:error, binary()}
  def drop_column(_, _), do: error()

  @spec inspect_table(window()) :: List.window()
  def inspect_table(_), do: error()

  @spec print(window()) :: no_return()
  def print(_), do: error()

  @spec stream_csv(pid(), binary()) :: no_return()
  defp stream_csv(_, _), do: error()

  @spec csv(binary(), fun()) :: :done
  def csv(file_path, func) when is_binary(file_path) and is_function(func) do
    stream_csv(self(), file_path)
    stream_read_loop(func)
    :done
  end

  @spec stream_read_loop(fun()) :: no_return()
  defp stream_read_loop(func) when is_function(func) do
    receive do
      msg when msg != nil ->
        func.(msg)
        stream_read_loop(func)

      nil ->
        nil
    end
  end

  def error, do: :erlang.nif_error(:nif_not_loaded)
end
