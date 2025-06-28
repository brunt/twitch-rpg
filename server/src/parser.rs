use crate::commands::PlayerCommand::Buy;
use crate::commands::{PlayerCommand, RpgCommand};
use crate::player_class::PlayerClass;
use winnow::Parser;
use winnow::ascii::{Caseless, alpha1, digit1, space1};
use winnow::combinator::{alt, preceded};
use winnow::error::EmptyError;

pub fn get_command(input: &mut &str) -> Option<RpgCommand> {
    preceded::<&str, &str, RpgCommand, EmptyError, &str, _>(
        "!",
        alt((
            Caseless("load").value(RpgCommand::Load),
            preceded("new", preceded(space1, parse_class.map(RpgCommand::New))),
            // preceded(
            //     "buy",
            //     preceded(
            //         space1,
            //         digit1.map(RpgCommand::PlayerCommand(Buy)),
            //     ),
            // ),
            // preceded(
            //     "use",
            //     preceded(
            //         space1,
            //         digit1.map(RpgCommand::PlayerCommand(Use)),
            //     ),
            // ),
        )),
    )
    .parse_next(input)
    .ok()
}

fn parse_class(input: &mut &str) -> Result<PlayerClass, EmptyError> {
    alt((
        Caseless("cleric").value(PlayerClass::Cleric),
        Caseless("druid").value(PlayerClass::Druid),
        Caseless("fighter").value(PlayerClass::Fighter),
        Caseless("monk").value(PlayerClass::Monk),
        Caseless("paladin").value(PlayerClass::Paladin),
        Caseless("ranger").value(PlayerClass::Ranger),
        Caseless("rogue").value(PlayerClass::Rogue),
        Caseless("warlock").value(PlayerClass::Warlock),
        Caseless("wizard").value(PlayerClass::Wizard),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_class() {
        let input = "wizard";
        assert_eq!(parse_class(&mut &*input), Ok(PlayerClass::Wizard));
    }

    #[test]
    fn test_parse_class_error() {
        let input = "other";
        assert_eq!(parse_class(&mut &*input), Err(EmptyError));
    }

    #[test]
    fn test_get_command() {
        let input = "!new wizard";
        assert_eq!(
            get_command(&mut &*input),
            Some(RpgCommand::New(PlayerClass::Wizard))
        );
    }

    #[test]
    fn test_get_command_load() {
        let input = "!load";
        assert_eq!(get_command(&mut &*input), Some(RpgCommand::Load));
    }
}
