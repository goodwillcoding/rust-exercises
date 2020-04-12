use rand::distributions::Distribution;
use rand::distributions::Standard;
use simple_error::SimpleError;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// The base type for representing Rock, Paper, and Scissors, which are all the
/// possible choices in our game.
#[derive(PartialEq, Eq)]
pub enum GameElement {
    Rock,
    Paper,
    Scissors,
}

/// Follows the standard ordering rules for rock paper scissors, where
/// rock < paper < scissors < rock.
///
/// # Examples
///
/// ```
/// use problem3::game_element::GameElement::*;
/// let comparison = Rock < Paper;
/// assert_eq!(comparison, true);
///```
impl Ord for GameElement {
    /// FIX ME!
    /// This allows users to compare Rock, Paper, and Scissors by defining
    /// the relationships between the three elements. e.g. Rock == Rock
    /// and Paper < Scissors.
    fn cmp(&self, other: &Self) -> Ordering {
        use GameElement::*;
        use Ordering::*;

        // This one is tricky. What are all the cases we need to cover? Do we have
        // tests for all the cases? The broken function below returns Less
        // for Rock compared to Paper (meaning `Rock < Paper == true`), but we
        // need to cover all cases.
        match (self, other) {
            // i have rock, they have paper -> lost
            (Rock, Paper) => Less,
            // i have rock, they have scissors -> won
            (Rock, Scissors) => Greater,
            // i have rock, they have rock -> draw
            (Rock, Rock) => Equal,

            // i have paper , they have scissors -> lost
            (Paper, Scissors) => Less,
            // i have paper, they have rock -> won
            (Paper, Rock) => Greater,
            // i have paper, they have rock -> draw
            (Paper, Paper) => Equal,

            // i have scissors, they have rock -> lost
            (Scissors, Rock) => Less,
            // i have scissors, they have rock -> won
            (Scissors, Paper) => Greater,
            // i have Scissors, they have rock -> draw
            (Scissors, Scissors) => Equal,
        }
    }
}

/// `Ord` requires that `PartialOrd` is implemented. `PartialOrd` returns
/// `Option<Ordering>` because some data types have values that cannot be
/// compared. Since `GameElement` should allow for all of its variants to be
/// compared, we can define the partial ordering via the `cmp` method from
/// `Ord`.
impl PartialOrd for GameElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Allows callers to randomly generate game choices.
impl Distribution<GameElement> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GameElement {
        use GameElement::*;
        // randomly chooses 1, 2, or 3
        let n: u32 = rng.gen_range(1, 4);

        match n {
            1 => Rock,
            2 => Paper,
            _ => Scissors,
        }
    }
}

/// Console-friendly string representation of each element.
impl fmt::Display for GameElement {
    /// FIX ME!
    /// This displays a user friendly string representation of all three
    /// `GameElement` variants.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Right now this always returns "Rock" no matter what element
        // we have. You can use `self` (an instance of `GameElement`) to
        // fix up our printer.
        let printable_str = match *self {
            GameElement::Rock => "Rock",
            GameElement::Paper => "Paper",
            GameElement::Scissors => "Scissors",
        };

        // The last line calls `write!` with the given formatter. You do not
        // need to modify it.
        write!(f, "{}", printable_str)
    }
}

/// For our game parser we'll accept any string that starts with r, p, or s
/// and convert it into Rock, Paper, or Scissors, respectively
impl FromStr for GameElement {
    type Err = SimpleError;

    /// FIX ME!
    /// Takes a string slice as input and either parses it into a `GameElement`
    /// or returns an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "r\n" {
            Ok(GameElement::Rock)
        } else if s == "p\n" {
            Ok(GameElement::Paper)
        } else if s == "s\n" {
            Ok(GameElement::Scissors)
        } else {
            Err(SimpleError::new("Choice must start with r, p, or s"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordering() {
        use GameElement::*;

        // does this test everything we need?
        assert!(Rock < Paper && Paper < Scissors && Scissors < Rock);
    }

    // add additional tests to make sure we can parse game elements from
    // strings and also display them
}
