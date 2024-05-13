defmodule TrebuchetTest do
  use ExUnit.Case
  doctest Trebuchet

  @data_file "./data/data.txt"

  test "runs test 1" do
    res = Trebuchet.solve(@data_file)

    IO.puts("\nPart 1: " <> Integer.to_string(res))

    assert res == 54953
  end

  test "runs test 2" do
    res = Trebuchet.solve(@data_file, test: 2)

    IO.puts("\nPart 2: " <> Integer.to_string(res))

    assert res == 53885
  end
end
