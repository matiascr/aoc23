defmodule MirageMaintenanceTest do
  use ExUnit.Case
  doctest MirageMaintenance

  import MirageMaintenance

  @example_file "./data/example.txt"
  @part_1_file "./data/data.txt"

  test "example" do
    result = part_1(@example_file)
    IO.puts("\nExample result: #{result}\n")

    assert result == 114
  end

  test "part 1" do
    result = part_1(@part_1_file)
    IO.puts("\nPart 1 result: #{result}\n")

    assert result == 2_043_677_056
  end
end
