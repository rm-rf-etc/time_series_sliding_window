defmodule SlidingWindowNif do
  use Rustler, otp_app: :sliding_window_nif, crate: "sliding_window_nif"
  @opaque t :: reference()

  @spec new(List.t(), integer(), integer()) :: {atom(), t() | binary()}
  def new(_, _, _), do: error()

  @spec push(t(), List.t()) :: {:ok, t()} | {:error, binary()}
  def push(_, _), do: error()

  @spec replace(t(), List.t()) :: {:ok, t()} | {:error, binary()}
  def replace(_, _), do: error()

  @spec add_column(t(), binary()) :: {:ok, t()} | {:error, binary()}
  def add_column(_, _), do: error()

  @spec drop_column(t(), binary()) :: {:ok, t()} | {:error, binary()}
  def drop_column(_, _), do: error()

  @spec inspect_table(t()) :: List.t()
  def inspect_table(_), do: error()

  @spec csv_start(binary()) :: no_return()
  def csv_start(_), do: error()

  @spec print(t()) :: no_return()
  def print(_), do: error()

  def error, do: :erlang.nif_error(:nif_not_loaded)
end
