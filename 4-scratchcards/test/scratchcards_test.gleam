import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import gleeunit
import gleeunit/should
import scratchcards.{scratch_pile}
import simplifile.{read}

type ReadError {
  FileError
}

pub fn main() {
  gleeunit.main()
}

fn get_lines(from filepath: String) -> Result(List(String), ReadError) {
  case read(from: filepath) {
    Ok(content) ->
      content
      |> string.trim()
      |> string.split(on: "\n")
      |> Ok()

    Error(_) -> Error(FileError)
  }
}

// gleeunit test functions end in `_test`

pub fn example_test() {
  let example_file = "data/example.txt"
  let assert Ok(lines) = get_lines(from: example_file)

  let result =
    scratch_pile(lines)
    |> list.reduce(fn(acc, x) { acc + x })
    |> result.unwrap(0)

  io.debug("Example result: " <> int.to_string(result))

  result
  |> should.equal(13)
}

pub fn part_1_test() {
  let data_file = "data/data.txt"
  let assert Ok(lines) = get_lines(from: data_file)

  let result =
    scratch_pile(lines)
    |> list.reduce(fn(acc, x) { acc + x })
    |> result.unwrap(0)

  io.debug("Part 1 result: " <> int.to_string(result))

  should.be_true(result > 4712)
}
