defmodule SlidingWindowNif do
  use Rustler, otp_app: :sliding_window_nif, crate: "sliding_window_nif"

  def add(_a, _b), do: error()
  def new(_n), do: error()
  def read(_n), do: error()
  def update(_a, _b), do: error()
  def error, do: :erlang.nif_error(:nif_not_loaded)
end
