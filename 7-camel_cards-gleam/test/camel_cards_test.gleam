import camel_cards.{part_1}
import gleam/int
import gleam/io
import gleeunit
import gleeunit/should
import simplifile

const example_file = "data/example.txt"

const data_file = "data/data.txt"

pub fn main() {
  gleeunit.main()
}

pub fn example_test() {
  let assert Ok(contents) = simplifile.read(from: example_file)

  let result = part_1(with: contents)

  io.debug("Example result: " <> int.to_string(result))

  result
  |> should.equal(6440)
}

pub fn part_1_test() {
  let assert Ok(contents) = simplifile.read(from: data_file)

  let result = part_1(with: contents)

  io.debug("Part 1 result: " <> int.to_string(result))

  result
  |> should.equal(249_638_405)
}
