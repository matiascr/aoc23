defmodule Trebuchet do
  def run(path) do
    File.read!(path)
    |> String.trim()
    |> String.split("\n")
    |> List.foldr(0, fn line, acc ->
      acc + find_calibration_value(line)
    end)
  end

  def find_calibration_value(line) do
    numbers =
      line
      |> String.graphemes()
      |> Enum.filter(fn grapheme ->
        case Integer.parse(grapheme) do
          {_number, ""} -> true
          {_number, _} -> false
          :error -> false
        end
      end)

    (List.first(numbers) <> List.last(numbers))
    |> String.to_integer()
  end
end

System.argv()
|> Enum.at(0)
|> Trebuchet.run()
|> IO.puts()
