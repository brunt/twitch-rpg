use crate::commands::{Item, PlayerClass, RpgCommand};
use winnow::Parser;
use winnow::ascii::{Caseless, alpha1, space1};
use winnow::combinator::{alt, preceded};
use winnow::error::EmptyError;

pub fn get_command(input: &mut &str) -> Option<RpgCommand> {
    preceded::<&str, &str, RpgCommand, EmptyError, &str, _>(
        "!",
        alt((
            Caseless("load").value(RpgCommand::Load),
            preceded(
                "new",
                preceded(
                    space1,
                    alpha1.map(|s: &str| {
                        PlayerClass::try_from(s.trim().to_string().parse())
                            .ok()
                            .map(|pc| RpgCommand::New(pc))
                    }),
                ),
            ),
            //TODO: plan this out
            // preceded(
            //     "buy",
            //     space1,
            //     alpha1.map(|s: &str|
            //         Item::try_from(s.trim()).ok().map(|item| item)
            //     )
            // )
            // )),
        )),
    )
    .parse_next(input)
    .ok()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_get_command() {
//         let input = "!play";
//         assert_eq!(get_command(&mut &*input), Some(RpgCommand::Play));
//     }
//
//     #[test]
//     fn test_new_name() {
//         let input = "!new bob";
//         assert_eq!(
//             get_command(&mut &*input),
//             Some(RpgCommand::New("bob".to_string()))
//         );
//     }
//
//     #[test]
//     fn test_new_name_spaces() {
//         let input = "!new   bob";
//         assert_eq!(
//             get_command(&mut &*input),
//             Some(RpgCommand::New("bob".to_string()))
//         );
//     }
// }
