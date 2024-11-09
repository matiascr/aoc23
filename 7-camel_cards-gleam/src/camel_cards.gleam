import gleam/int
import gleam/list.{Continue, Stop}
import gleam/option.{type Option, None, Some}
import gleam/order.{type Order, Eq, Gt, Lt}
import gleam/result
import gleam/string

type Bid =
  Int

type Hand =
  List(Int)

type HandType {
  HighCard
  OnePair
  TwoPair
  ThreeOAK
  FourOAK
  FiveOAK
  FullHouse
}

type Play =
  #(HandType, Hand, Bid)

fn parse_card(card_string: String) -> Result(Int, String) {
  case int.parse(card_string) {
    Ok(val) if val < 10 && val > 1 -> Ok(val)
    Ok(_) -> Error("Not a valid value.")
    Error(_) ->
      case card_string {
        "T" -> Ok(10)
        "J" -> Ok(11)
        "Q" -> Ok(12)
        "K" -> Ok(13)
        "A" -> Ok(14)
        _ -> Error("Failed to parse card.")
      }
  }
}

fn hand_type_value(hand_type: HandType) -> Int {
  case hand_type {
    HighCard -> 1
    OnePair -> 2
    TwoPair -> 3
    ThreeOAK -> 4
    FullHouse -> 5
    FourOAK -> 6
    FiveOAK -> 7
  }
}

fn hand_to_hand_type(hand: Hand) -> HandType {
  case list.sort(hand, int.compare) {
    [a, b, c, d, e] if a == b && a == c && a == d && a == e -> FiveOAK

    [a, b, c, d, e] if a == b && a == c && a == d && a != e -> FourOAK
    [a, b, c, d, e] if a != b && b == c && b == d && b == e -> FourOAK

    [a, b, c, d, e] if a == b && a == c && d == e -> FullHouse
    [a, b, c, d, e] if a == b && c == d && c == e -> FullHouse

    [a, b, c, _, _] if a == b && a == c -> ThreeOAK
    [_, b, c, d, _] if b == c && b == d -> ThreeOAK
    [_, _, c, d, e] if c == d && d == e -> ThreeOAK

    [a, b, c, d, _] if a == b && c == d -> TwoPair
    [_, b, c, d, e] if b == c && d == e -> TwoPair
    [a, b, _, d, e] if a == b && d == e -> TwoPair

    [a, b, _, _, _] if a == b -> OnePair
    [_, b, c, _, _] if b == c -> OnePair
    [_, _, c, d, _] if c == d -> OnePair
    [_, _, _, d, e] if d == e -> OnePair

    _ -> HighCard
  }
}

fn string_to_hand(hand_string: String) -> Hand {
  hand_string
  |> string.to_graphemes()
  |> list.map(parse_card)
  |> result.all()
  |> result.unwrap(or: [])
}

fn get_play(play_string: String) -> Option(Play) {
  case string.split(play_string, " ") {
    [hand_string, bid_string] -> {
      let assert Ok(bid) = int.parse(bid_string)
      let hand = string_to_hand(hand_string)
      let hand_type = hand_to_hand_type(hand)

      Some(#(hand_type, hand, bid))
    }
    _ -> None
  }
}

fn compare_int_list(a: List(Int), with b: List(Int)) -> Order {
  list.zip(a, b)
  |> list.fold_until(Eq, fn(_, value) {
    let #(aa, bb) = value
    case aa, bb {
      aa, bb if aa < bb -> Stop(Lt)
      aa, bb if aa > bb -> Stop(Gt)
      _, _ -> Continue(Eq)
    }
  })
}

fn compare_hand_types(a: HandType, b: HandType) -> Order {
  let a_value = hand_type_value(a)
  let b_value = hand_type_value(b)

  int.compare(a_value, b_value)
}

fn hand_compare(p1: Play, p2: Play) -> Order {
  let #(hand_type_1, hand_1, _) = p1
  let #(hand_type_2, hand_2, _) = p2

  case compare_hand_types(hand_type_1, hand_type_2) {
    Eq -> compare_int_list(hand_1, hand_2)
    order -> order
  }
}

fn play_winnings(play: Play, index: Int) -> Int {
  let #(_hand_type, _hand, bid) = play
  { bid * { index + 1 } }
}

fn multiply_by_rank(hands: List(Play)) -> Int {
  list.index_fold(over: hands, from: 0, with: fn(acc, play, index) {
    { play_winnings(play, index) + acc }
  })
}

pub fn part_1(with input: String) -> Int {
  input
  |> string.split(on: "\n")
  |> list.filter(fn(substring) { substring != "" })
  |> list.map(get_play)
  |> option.values()
  |> list.sort(hand_compare)
  |> multiply_by_rank()
}
