use rand::distributions::Distribution;
use rand::distributions::Standard;
use simple_error::SimpleError;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// The base type for representing Rock, Paper, and Scissors, which are all the
/// possible choices in our game.
#[derive(PartialEq, Eq)]
// add Debug trait for testing
#[derive(Debug)]
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
            // self has rock, other has paper -> lost
            (Rock, Paper) => Less,
            // self has rock, other scissors -> won
            (Rock, Scissors) => Greater,
            // self has  rock, other has rock -> draw
            (Rock, Rock) => Equal,

            // self has paper , other has scissors -> lost
            (Paper, Scissors) => Less,
            // self has paper, other has rock -> won
            (Paper, Rock) => Greater,
            // self has paper, other has paper -> draw
            (Paper, Paper) => Equal,

            // self has scissors, other has rock -> lost
            (Scissors, Rock) => Less,
            // self has scissors, other paper -> won
            (Scissors, Paper) => Greater,
            // self has scissors, other has scissors -> draw
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
        // randomly chooses 1, 2, or 3
        let n: u32 = rng.gen_range(1, 4);

        match n {
            1 => GameElement::Rock,
            2 => GameElement::Paper,
            _ => GameElement::Scissors,
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

        // rock loses to paper
        assert!(Rock < Paper);
        // rock  beats scissors
        assert!(Rock > Scissors);
        // rock and rock is a draw
        assert!(Rock == Rock);

        // paper loses to scissors
        assert!(Paper < Scissors);
        // paper beats rock
        assert!(Paper > Rock);
        // paper and paper is a draw
        assert!(Paper == Paper);

        // scissors loses to rock
        assert!(Scissors < Rock);
        // scissors beats rock
        assert!(Scissors > Paper);
        // scissors and scissors is a draw
        assert!(Scissors == Scissors);
    }

    // add additional tests to make sure we can parse game elements from
    // strings and also display them

    #[test]
    fn test_cmp() {
        use std::cmp::Ordering::*;
        use GameElement::*;

        // alternative tests using cmp directly

        // rock loses to paper
        assert_eq!(Rock.cmp(&Paper), Less);
        // rock  beats scissors
        assert_eq!(Rock.cmp(&Scissors), Greater);
        // // rock and rock is a draw
        assert_eq!(Rock.cmp(&Rock), Equal);

        // paper loses to scissors
        assert_eq!(Paper.cmp(&Scissors), Less);
        // paper beats rock
        assert_eq!(Paper.cmp(&Rock), Greater);
        // paper and paper is a draw
        assert_eq!(Paper.cmp(&Paper), Equal);

        // // scissors loses to rock
        assert_eq!(Scissors.cmp(&Rock), Less);
        // // scissors beates rock
        assert_eq!(Scissors.cmp(&Paper), Greater);
        // scissors and scissors is a draw
        assert_eq!(Scissors.cmp(&Scissors), Equal);
    }

    #[test]
    fn test_partial_cmp() {
        use std::cmp::Ordering::*;
        use GameElement::*;

        // alternative tests using cmp directly

        // rock loses to paper
        assert_eq!(Rock.partial_cmp(&Paper), Some(Less));
        // rock  beats scissors
        assert_eq!(Rock.partial_cmp(&Scissors), Some(Greater));
        // // rock and rock is a draw
        assert_eq!(Rock.partial_cmp(&Rock), Some(Equal));

        // paper loses to scissors
        assert_eq!(Paper.partial_cmp(&Scissors), Some(Less));
        // paper beats rock
        assert_eq!(Paper.partial_cmp(&Rock), Some(Greater));
        // paper and paper is a draw
        assert_eq!(Paper.partial_cmp(&Paper), Some(Equal));

        // // scissors loses to rock
        assert_eq!(Scissors.partial_cmp(&Rock), Some(Less));
        // // scissors beates rock
        assert_eq!(Scissors.partial_cmp(&Paper), Some(Greater));
        // scissors and scissors is a draw
        assert_eq!(Scissors.partial_cmp(&Scissors), Some(Equal));
    }

    #[test]
    fn test_sample() {
        use rand::rngs::mock::StepRng;
        use GameElement::*;

        let mut mock_rng = StepRng::new(1, 1);
        let elements: [GameElement; 3] = Standard.sample(&mut mock_rng);
        // as far as I can tell StepRng does not implement range_gen
        // so range_gen seems to always return 1 and does not
        // increament on follow up calls, so we get basically [1,1,1] which
        // translates to Rock
        assert_eq!(elements, [Rock, Rock, Rock]);
    }

    #[test]
    fn test_fmt_display() {
        use GameElement::*;

        assert_eq!(format!("{}", Rock), "Rock");
        assert_eq!(format!("{}", Paper), "Paper");
        assert_eq!(format!("{}", Scissors), "Scissors");
    }

    #[test]
    fn test_from_str() {
        let rock = GameElement::from_str("r\n");
        assert_eq!(rock.unwrap(), GameElement::Rock);

        let paper = GameElement::from_str("p\n");
        assert_eq!(paper.unwrap(), GameElement::Paper);

        let scissors = GameElement::from_str("s\n");
        assert_eq!(scissors.unwrap(), GameElement::Scissors);
    }
}
