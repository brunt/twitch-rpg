use crate::commands::PlayerCommand::{Buy, Show, Use};
use crate::commands::RpgCommand;
use common::{MenuItem, PlayerClass};
use winnow::Parser;
use winnow::ascii::{Caseless, alpha1, digit1, space1};
use winnow::combinator::{alt, preceded};
use winnow::error::EmptyError;
use crate::commands::RpgCommand::PlayerCommand;

pub fn get_command(input: &mut &str) -> Option<RpgCommand> {
    preceded::<&str, &str, RpgCommand, EmptyError, &str, _>(
        "!",
        alt((
            Caseless("show").value(PlayerCommand(Show)),
            Caseless("rejoin").value(RpgCommand::Rejoin),
            preceded("join", preceded(space1, parse_class.map(RpgCommand::Join))),
            preceded(
                "buy",
                preceded(
                    space1,
                    digit1.map(|number: &str| {
                        PlayerCommand(Buy(MenuItem::from(
                            number.parse::<usize>().unwrap_or_default(),
                        )))
                    }),
                ),
            ),
            preceded(
                "use",
                preceded(
                    space1,
                    digit1.map(|number: &str| {
                        PlayerCommand(Use(MenuItem::from(
                            number.parse::<usize>().unwrap_or_default(),
                        )))
                    }),
                ),
            ),
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
        // Caseless("monk").value(PlayerClass::Monk),
        Caseless("paladin").value(PlayerClass::Paladin),
        Caseless("ranger").value(PlayerClass::Ranger),
        Caseless("rogue").value(PlayerClass::Rogue),
        Caseless("warlock").value(PlayerClass::Warlock),
        Caseless("wizard").value(PlayerClass::Wizard),
        Caseless("sorcerer").value(PlayerClass::Sorcerer),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::MenuItem;
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
        let input = "!join wizard";
        assert_eq!(
            get_command(&mut &*input),
            Some(RpgCommand::Join(PlayerClass::Wizard))
        );
    }

    #[test]
    fn test_get_command_load() {
        let input = "!rejoin";
        assert_eq!(get_command(&mut &*input), Some(RpgCommand::Rejoin));
    }

    #[test]
    fn test_buy_item() {
        let input = "!buy 1";
        assert_eq!(
            get_command(&mut &*input),
            Some(RpgCommand::PlayerCommand(Buy(MenuItem::from(1))))
        );
    }

    #[test]
    fn test_use_item() {
        let input = "!use 1";
        assert_eq!(
            get_command(&mut &*input),
            Some(PlayerCommand(Use(MenuItem::from(1))))
        );
    }
    
    #[test]
    fn test_show_command() {
        let input = "!show";
        assert_eq!(
            get_command(&mut &*input),
            Some(PlayerCommand(Show))
        );
    }
}
