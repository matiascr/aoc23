defmodule Trebuchet do
  @moduledoc """
  Documentation for Exercise `Trebuchet` of Advent of Code 2023.

  [Advent of Code 2023 - Day 1](https://adventofcode.com/2023/day/1)
  """

  @number_digits ~w(1 2 3 4 5 6 7 8 9 0)
  @number_strings ~w(one two three four five six seven eight nine)

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
    for n <- String.graphemes(line), into: [] do
      if Enum.member?(@number_digits, n) do
        Enum.find_index(@number_digits, &(&1 == n))
        |> then(&Enum.at(@number_digits, &1))
        |> String.to_integer()
      end
    end
    |> Enum.filter(&(&1 != nil))
  end

  def parse_calibration_values(line, 2) do
    line
    |> String.graphemes()
    |> Enum.reduce([], fn grapheme, acc ->
      prev_chars = if match?([_h | _t], acc), do: Enum.at(acc, 0), else: ""

      cond do
        Enum.member?(@number_digits, grapheme) ->
          [String.to_integer(grapheme) | acc]

        is_integer(prev_chars) ->
          [grapheme | acc]

        has_integer_string(prev_chars <> grapheme) ->
          [integer_in_string(prev_chars <> grapheme) | Enum.slice(acc, 1..-1//1)]

        true ->
          [prev_chars <> grapheme | Enum.slice(acc, 1..-1//1)]
      end
    end)
    |> Enum.filter(&is_integer/1)
    |> Enum.reverse()
  end

  @spec has_integer_string(binary()) :: boolean()
  def has_integer_string(str) when is_binary(str) do
    Enum.find(@number_strings, fn ns ->
      match?({_start, _length}, :binary.match(str, ns))
    end) != nil
  end

  @spec integer_in_string(binary()) :: [non_neg_integer()]
  def integer_in_string(str) when is_binary(str) do
    @number_strings
    |> Enum.with_index()
    |> Enum.map(fn {ns, i} ->
      case :binary.match(str, ns) do
        {_start, _length} -> i + 1
        _ -> nil
      end
    end)
    |> Enum.filter(&(&1 != nil))
    |> Enum.at(0)
  end
end
