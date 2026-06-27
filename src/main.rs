use std::fmt::Display;

struct Question {
    text: &'static str,
    choices: &'static [(&'static str, Questions)],
}

impl Question {
    fn next(&self, index: usize) -> Option<Questions> {
        self.choices.get(index).map(|(_, question)| *question)
    }
}

const WELCOME_MESSAGE: &str = r#"Welkom bij de ""Build your own adventure game"".
Je kunt ten aller tijde '0' typen om het spel af te sluiten.
"#;

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.text)?;
        writeln!(f, "Wat doe je?")?;
        for (index, (choice, _)) in self.choices.iter().enumerate() {
            writeln!(f, "{}. {choice}", index + 1)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
enum Questions {
    Start,
    KillElf,
    SneakToElf,
    WaitForElf,
    LootElf,
    ChaseNoise,
    KeepListening,
    WalkForward,
    TurnAround,
    Finish,
}

impl Questions {
    fn get(&self) -> Question {
        match self {
            Self::Start => Question {
                text: r#"Je bevind je aan de rand van een bos. Je loopt het bos in langs een bospad.
Verderop het pad zie je een elf met zijn rug naar je toe staan."#,
                choices: &[
                    ("Val aan", Self::KillElf),
                    ("Sluip voorzichtig dichterbij", Self::SneakToElf),
                    ("Wacht rustig af", Self::WaitForElf),
                ],
            },
            Self::KillElf => Question {
                text: r#"Je rent op de niets vermoedende elf af en steekt hem in de rug met een mes.
                Hij zakt kreunend in elkaar. Op dat moment hoor je geritsel in de bosjes naast het pad.
                Er rent iemand weg; dieper het bos in."#,
                choices: &[
                    ("Loot de elf", Self::LootElf),
                    ("Volg het geritsel", Self::ChaseNoise),
                ],
            },
            Self::SneakToElf => Question {
                text: r#"Je sluipt stilletjes naar de elf toe en verstopt je langs de zijkant van het pad.
                Je hoort nu dat de elf in gesprek is met iemand in de bosjes. De elf zegt:
                "Als iemand hier achter komt moeten we hem vermoorden"."#,
                choices: &[
                    ("Val de elf aan", Self::KillElf),
                    ("Blijf luisteren", Self::KeepListening),
                ],
            },
            Self::WaitForElf => Question {
                text: r#"Na een tijdje kijkt de elf achterom en jullie kijken elkaar aan.
                De elf draait zich om en loopt verder langs het pad, dieper het bos in."#,
                choices: &[
                    ("Vervolg de weg het bos in", Self::WalkForward),
                    ("Keer om", Self::TurnAround),
                ],
            },
            Self::LootElf => Question {
                text: r#""#,
                choices: &[
                    ("Loot de elf", Self::LootElf),
                    ("Volg het geritsel", Self::ChaseNoise),
                ],
            },
            Self::ChaseNoise => Question {
                text: r#""#,
                choices: &[
                    ("Loot de elf", Self::LootElf),
                    ("Volg het geritsel", Self::Finish),
                ],
            },
            Self::Finish => Question {
                text: r#"Je hebt het einde van dit avontuur bereikt."#,
                choices: &[("Opnieuw spelen", Self::Start)],
            },
            // TODO Verwijder deze arm voor release
            _ => Question {
                text: r#"Dit is nog niet geimplementeerd!"#,
                choices: &[],
            },
        }
    }
}

fn print_screen(question: &Question) {
    print!("\x1B[2J\x1B[1;1H"); // Clear the screen and put cursor at (1,1)
    print!("{}", question);
    println!("0. Afsluiten");
    println!("> ");
}

fn main() {
    println!("{WELCOME_MESSAGE}");
    let mut question = Questions::Start.get();
    let stdin = std::io::stdin();
    print_screen(&question);
    for line in stdin.lines() {
        let line = line.expect("Connection to terminal lost");
        question = match line.trim().parse::<u8>() {
            Ok(0) => {
                println!("Tot ziens.");
                break;
            }
            Ok(choice) => {
                let index = (choice - 1) as usize;
                if let Some(next) = question.next(index) {
                    next.get()
                } else {
                    println!("Dat is geen geldige keuze. Probeer het opnieuw.");
                    continue;
                }
            }
            Err(_) => {
                println!("Dat is geen geldige keuze. Probeer het opnieuw.");
                continue;
            }
        };
        print_screen(&question);
    }
}
