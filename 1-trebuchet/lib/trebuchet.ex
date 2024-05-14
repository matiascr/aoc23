defmodule Trebuchet do
  @moduledoc """
  Documentation for Exercise `Trebuchet` of Advent of Code 2023.

  [Advent of Code 2023 - Day 1](https://adventofcode.com/2023/day/1)
  """

  @number_digits ~w(one1one two2two three3three four4four five5five six6six seven7seven eight8eight nine9nine)
  @number_strings ~w(one two three four five six seven eight nine)
  @mapping Enum.zip(@number_strings, @number_digits)

  @spec solve(binary(), keyword()) :: non_neg_integer()
  def solve(path, opts \\ []) when is_list(opts) do
    test_n = Keyword.get(opts, :test, 1)

    File.read!(path)
    |> String.trim()
    |> String.split(~r(\r\n|\r|\n))
    |> Enum.map(fn line ->
      parsed = parse_calibration_values(line, test_n)

      Enum.at(parsed, 0) * 10 + Enum.at(parsed, -1)
    end)
    |> Enum.sum()
  end

  @spec parse_calibration_values(binary(), pos_integer()) :: [non_neg_integer()]
  def parse_calibration_values(line, 1) do
    String.graphemes(line)
    |> Enum.filter(&(Integer.parse(&1) != :error))
    |> Enum.map(&String.to_integer/1)
  end

  def parse_calibration_values(line, 2) do
    Enum.reduce(@mapping, line, fn {str, int}, acc ->
      String.replace(acc, str, int)
    end)
    |> parse_calibration_values(1)
  end
end
