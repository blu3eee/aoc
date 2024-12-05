## Please find the problem at the Advent of Code website

[Advent of Code - Day 07](https://adventofcode.com/2023/day/7)

## Solution & Thoughts

**Normalizing data:** after reading the input from input.txt, return it to a vector of String (lines), I continued to normalize it down to each hand and bid for each line.

**Sorting the cards:**

- I use quicksort to sort the Hand base on its score we got from `get_hand_score` function, if the score is equal, we then compare the two Hands cards one by one starting from the first card. At this point, we actually need to convert the hand string into a Vector of cards point, 2 -> 2, 3 -> 3, ..., T -> 10, .., A -> 14. The reason we doing this is that when comparing the characters, T is actually higher than K in the alphabet order so we convert the card it its point for more accurate comparison. So when we comparing the two Hands cards, we actually comparing the vector of card points (still in the same order of the input hand).

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    pub score: usize,
    pub hand_cards: Vec<usize>,
    pub bid: u32,
    pub hand: String, // for debugging purposes
}

impl Ord for Hand {
    // only compare the hand_cards if the score is EQUAL
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.score, &self.hand_cards).cmp(&(&other.score, &other.hand_cards))
    }
}
```

### PART 1

**Get Hand Score:**

- With the problem description for part 1 at [Advent of Code - Day 07](https://adventofcode.com/2023/day/7), you can see that we have different kind of hands and each hand will have its strength base on the hand cards, so I have a function `get_hand_score` to get the **score** of the hand. There are `7 different kinds of hands` for this problem, so I have the algorithm returning the score of 1 to 7. The implementation of `get_hand_score` for part 1 is pretty straightforward, you iterate through each characters and count it, save it in a `HashMap` mapping the character to its count.
- After that, we based on how many different cards we have on our hand (the length of the `HashMap` keys) to determine the score.
  - `5` different cards is **[High card]** hand
  - `4` different cards is **[One pair]** hand
  - `3` different cards, the hand is either **[Two pair]** or **[Three of a kind]** depends on if there is a card with the count of `3`.
  - `2` different cards, the hand is either **[Full house]** or **[Four of a kind]** depends on if there is a card with the count of `4`.
  - `1` type of card: **[Five of a kind]** hand

### PART 2

- For part 2, we consider the card `J` as a `Joker` card, which will act as a wild Card can match with any other Card to help us make a stronger hand (higher score), but it will be a lowest card when we comparing each card individually. We take this into account when we calculate the hand's score in the `get_hand_score_p2` function. The nature of the problem is the same, calculate the hand's score, rank it with other hands.
- When determing the cards: after we count the different cards on our hand, we also count the number of joker card(s) on our hand. To see into details how I take the joker card (with documentation) into consideration, please visit main.rs in the `src` folder of this module.

### THOUGHTS

- This challenge is pretty interesting as I got the chance to implement the Quick Sort algorithm again in Rust, refresh my mind with some of these algorithm.

- Reading through the problem and considering all possible cases with the joker is fun.

---
