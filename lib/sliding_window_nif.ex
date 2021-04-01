defmodule SlidingWindowNif do
  use Rustler, otp_app: :sliding_window_nif, crate: "sliding_window_nif"
  def new(_, _), do: error()
  def push(_, _), do: error()
  def replace(_, _), do: error()
  def add_column(_, _), do: error()
  def drop_column(_, _), do: error()
  def print(_), do: error()
  def error, do: :erlang.nif_error(:nif_not_loaded)
end
