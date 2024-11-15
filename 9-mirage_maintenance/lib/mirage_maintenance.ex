defmodule MirageMaintenance do
  @moduledoc false

  @doc """
    iex> MirageMaintenance.line_to_history("1 3 6 10 15 21")
    [1, 3, 6, 10, 15, 21]

    iex> MirageMaintenance.line_to_history("0 3 6 9 12 15")
    [0, 3, 6, 9, 12, 15]

    iex> MirageMaintenance.line_to_history("3 3 3 3 3")
    [3, 3, 3, 3, 3]
  """
  @spec line_to_history(String.t()) :: [integer()]
  def line_to_history(line) do
    line
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end

  @doc """
    iex> MirageMaintenance.get_diffs([1, 3, 6, 10, 15, 21])
    [2, 3, 4, 5, 6]

    iex> MirageMaintenance.get_diffs([0, 3, 6, 9, 12, 15])
    [3, 3, 3, 3, 3]

    iex> MirageMaintenance.get_diffs([3, 3, 3, 3, 3])
    [0, 0, 0, 0]
  """
  @spec get_diffs([integer()]) :: [integer()]
  def get_diffs([h | _] = line) when is_list(line) and is_integer(h) do
    {[_ | t], _} = Enum.map_reduce(line, 0, fn elem, acc -> {elem - acc, elem} end)
    t
  end

  @doc """
    iex> MirageMaintenance.expand_history([0, 3, 6, 9, 12, 15])
    [
      [0, 3, 6, 9, 12, 15],
      [3, 3, 3, 3, 3],
      [0, 0, 0, 0]
    ]

    iex> MirageMaintenance.expand_history([1, 3, 6, 10, 15, 21])
    [
      [1, 3, 6, 10, 15, 21],
      [2, 3, 4, 5, 6],
      [1, 1, 1, 1],
      [0, 0, 0]
    ]
  """
  @spec expand_history([integer()]) :: [[integer()]]
  def expand_history([h | _] = history) when is_list(history) and is_integer(h) do
    0..length(history)
    |> Enum.reduce_while([history], fn _, acc ->
      last = acc |> Enum.at(-1)

      last
      |> Enum.uniq()
      |> case do
        [0] -> {:halt, acc}
        _ -> {:cont, acc ++ [get_diffs(last)]}
      end
    end)
  end

  @doc """
    iex> MirageMaintenance.append_next([0, 3, 6, 9, 12, 15])
    [
      [0, 3, 6, 9, 12, 15, 18],
      [3, 3, 3, 3, 3, 3],
      [0, 0, 0, 0, 0]
    ]
  """
  @spec append_next([integer()]) :: [[integer()]]
  def append_next(history) when is_list(history) do
    history
    |> expand_history()
    |> Enum.reverse()
    |> Enum.map_reduce([], fn elem, acc ->
      elem =
        case length(Enum.uniq(elem)) do
          1 -> [Enum.at(elem, 0) | elem]
          _ -> elem ++ [Enum.at(elem, -1) + Enum.at(acc, -1)]
        end

      {elem, elem}
    end)
    |> Tuple.to_list()
    |> Enum.at(0)
    |> Enum.reverse()
  end

  @doc """
    iex> MirageMaintenance.get_latest([
    ...>   [0, 3, 6, 9, 12, 15, 18],
    ...>   [3, 3, 3, 3, 3, 3],
    ...>   [0, 0, 0, 0, 0]
    ...> ])
    18
  """
  @spec get_latest([[integer()]]) :: integer()
  def get_latest(expanded_history) when is_list(expanded_history) do
    expanded_history |> Enum.at(0) |> Enum.at(-1)
  end

  @spec part_1(String.t()) :: number()
  def part_1(file_path) do
    {:ok, contents} = File.read(file_path)

    contents
    |> String.split("\n")
    |> Enum.filter(fn line -> line != "" end)
    |> Enum.map(fn history ->
      history
      |> line_to_history()
      |> append_next()
      |> get_latest()
    end)
    |> Enum.sum()
  end
end
