import gleam/float
import gleam/int
import gleam/list
import gleam/regex
import gleam/result
import gleam/string

type Card {
  Card(card_number: Int, winning_numbers: List(Int), numbers: List(Int))
}

fn create_card(card_string: String) -> Card {
  let assert Ok(re) = regex.from_string(":|\\|")

  let assert [card_number, winner_numbers, numbers] =
    regex.split(with: re, content: card_string)

  Card(
    card_number: string.replace(card_number, "Card ", "")
      |> int.parse()
      |> result.unwrap(or: 0),
    winning_numbers: winner_numbers
      |> string.split(" ")
      |> list.map(int.parse(_))
      |> result.values(),
    numbers: numbers
      |> string.split(" ")
      |> list.map(int.parse(_))
      |> result.values(),
  )
}

fn scratch_card(card: Card) -> List(Int) {
  card.numbers
  |> list.filter(list.contains(card.winning_numbers, _))
}

fn count_winners(numbers: List(Int)) -> Int {
  case numbers {
    [] -> 0
    _ ->
      list.length(numbers) - 1
      |> int.to_float()
      |> int.power(2, _)
      |> result.unwrap(0.0)
      |> float.round()
  }
}

pub fn scratch_pile(pile: List(String)) -> List(Int) {
  pile
  |> list.map(create_card(_))
  |> list.map(scratch_card(_))
  |> list.map(count_winners(_))
}
