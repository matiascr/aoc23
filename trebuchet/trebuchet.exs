defmodule Trebuchet do
  def calculate(path) do
    File.read!(path)
    |> String.trim()
    |> String.split("\n")
    |> List.foldr(0, fn line, acc ->
      acc + calibration_value(line)
    end)
  end

  def calibration_value(line) do
    numbers =
      line
      |> String.graphemes()
      |> Enum.filter(&grapheme_is_integer?/1)

    (List.first(numbers) <> List.last(numbers))
    |> String.to_integer()
  end

  def grapheme_is_integer?(grapheme) when is_binary(grapheme) do
    case Integer.parse(grapheme) do
      {_number, ""} -> true
      {_number, _} -> false
      :error -> false
    end
  end
end

System.argv()
|> Enum.at(0)
|> Trebuchet.calculate()
|> IO.puts()
