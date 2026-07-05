use std::{fmt::Display, io::Write, time::Duration};

struct Question {
    text: &'static str,
    choices: &'static [(&'static str, Questions)],
}

impl Question {
    fn next(&self, index: usize) -> Option<Questions> {
        self.choices.get(index).map(|(_, question)| *question)
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.text)?;
        writeln!(f, "Wat doe je?")?;
        for (index, (choice, _)) in self.choices.iter().enumerate() {
            writeln!(f, "{}) {choice}", index + 1)?;
        }
        writeln!(f, "0) Afsluiten")
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

    MeetHunter,
    FollowHunter,
    DistrustHunter,

    HiddenCamp,
    SearchCamp,
    FreePrisoner,

    PrisonerStory,
    IgnorePrisoner,

    AncientRoad,
    OldBridge,
    RiverBank,

    Hermit,
    TalkHermit,
    AttackHermit,

    RuinsEntrance,
    SecretPassage,
    FrontGate,

    VaultHall,
    MuralRoom,
    HiddenLibrary,
    AncientMachine,

    GhostGuardian,
    AnswerGuardian,
    FightGuardian,

    UndergroundLake,
    BoatCrossing,
    DeepTunnel,

    ElfCouncil,
    HumanCaptain,
    ChooseSide,

    EscapeForest,
    BecomeGuardian,
    TakeCrown,
    DestroyCrown,
    LeaveWithTruth,

    MemoryEcho,
    BrokenTimeHall,
    MirrorElf,
    MirrorHuman,
    FalseKing,
    TrueKing,
    ThePlayerMemory,

    TimeLoopStart,
    TimeLoopEnd,

    HiddenWarRoom,
    WarSimulationCore,

    TruthFragmentA,
    TruthFragmentB,
    TruthFragmentC,

    CollapseChoice,
    RebuildChoice,

    EndingLoop,
    EndingSimulation,
    EndingBecoming,
    EndingObserver,
    EndingErase,
    EndingSplitSelf,

    EndingEscape,
    EndingGuardian,
    EndingKing,
    EndingPeace,
    EndingDeath,

    TrueStart,
    FalseStart,
    LoopMemory,

    CrownCore,
    CrownInterface,
    CrownShard,

    SystemVoice,
    SystemBreak,
    SystemRewrite,

    EndingTrueStart,
    EndingLoopBreak,
    EndingSystemMerge,
    EndingPlayerErase,
    EndingFreedom,
    EndingReturn,
    EndingNoExit,

    Finish,
}

impl Questions {
    fn get(&self) -> Question {
        match self {
            Self::Start => Question {
                text: "Je staat aan de rand van het bos.\n\nMaar er is iets vreemds.\nJe herinnert je dit moment al.",
                choices: &[
                    ("Stap het bos in", Self::SneakToElf),
                    ("Blijf staan", Self::LoopMemory),
                    (
                        "Probeer je te herinneren wat hiervoor kwam",
                        Self::TruthFragmentA,
                    ),
                ],
            },
            Self::LootElf => Question {
                text: "Je doorzoekt de elf. Zijn beurs bevat slechts enkele zilverstukken, maar in zijn mantel vind je een zorgvuldig opgevouwen brief. Er staat slechts één zin op.\n\n'Bij zonsondergang. De oude ruïne. Vertrouw niemand.'\n\nPlotseling klinkt opnieuw geritsel.",
                choices: &[
                    ("Ren achter het geluid aan", Self::ChaseNoise),
                    ("Onderzoek de omgeving", Self::AncientRoad),
                    ("Keer terug naar het pad", Self::WalkForward),
                ],
            },

            Self::ChaseNoise => Question {
                text: "Je sprint tussen de bomen door. Takken slaan tegen je gezicht. Uiteindelijk zie je een jonge elf struikelen. Hij kijkt je verschrikt aan en laat direct zijn boog vallen.\n\n'Wacht! Ik vocht niet met hem!'",
                choices: &[
                    ("Luister naar hem", Self::PrisonerStory),
                    ("Grijp hem vast", Self::HiddenCamp),
                    ("Laat hem vluchten", Self::WalkForward),
                ],
            },

            Self::KeepListening => Question {
                text: "Je blijft luisteren.\n\nMaar de stemmen overlappen met een eerdere versie van dit gesprek.\n\nEén van hen zegt iets dat jij nog niet hebt meegemaakt.",
                choices: &[
                    ("Confronteer de herhaling", Self::MemoryEcho),
                    ("Aanvallen", Self::KillElf),
                    ("Terugtrekken", Self::WalkForward),
                ],
            },
            Self::WalkForward => Question {
                text: "Het pad slingert dieper het bos in. De vogels zijn stil. Na enkele minuten splits de weg zich. Links zie je een oude stenen brug. Rechts loopt een nauwelijks zichtbaar wildspoor langs een rivier.",
                choices: &[
                    ("Neem de brug", Self::OldBridge),
                    ("Volg de rivier", Self::RiverBank),
                    ("Keer terug", Self::TurnAround),
                ],
            },

            Self::TurnAround => Question {
                text: "Je besluit dat het verstandig is om terug te keren. Terwijl je richting de bosrand loopt zie je echter een zwaarbewapende jager jouw kant op komen. Hij lijkt je al van grote afstand te hebben gezien.",
                choices: &[
                    ("Begroet de jager", Self::MeetHunter),
                    ("Verstop je", Self::DistrustHunter),
                    ("Loop rustig door", Self::MeetHunter),
                ],
            },

            Self::MeetHunter => Question {
                text: "De jager kijkt je aan alsof hij een keuze herkent die je al vaker hebt gemaakt.\n\n'Je blijft terugkomen naar dit punt,' zegt hij.",
                choices: &[
                    ("Vraag waarom", Self::FollowHunter),
                    ("Ontken het", Self::DistrustHunter),
                    ("Aanvallen", Self::KillElf),
                ],
            },

            Self::FollowHunter => Question {
                text: "Je volgt hem opnieuw.\n\nMaar deze keer lijkt het pad korter. Alsof je het al eens hebt gelopen.",
                choices: &[
                    ("Blijven volgen", Self::HiddenCamp),
                    ("Stoppen", Self::AncientRoad),
                ],
            },

            Self::DistrustHunter => Question {
                text: "Je vertrouwt hem niet.\n\nHij knikt langzaam.\n\n'Logisch. Dat is meestal de derde variant van deze ontmoeting.'",
                choices: &[
                    ("Volg hem toch", Self::FollowHunter),
                    ("Ga alleen verder", Self::AncientRoad),
                ],
            },

            Self::HiddenCamp => Question {
                text: "Diep tussen de bomen ligt een verborgen kamp. Er staan slechts drie tenten, maar overal zie je sporen van haastig vertrek. Midden op het terrein zit iemand vastgebonden aan een boom.",
                choices: &[
                    ("Doorzoek het kamp", Self::SearchCamp),
                    ("Bevrijd de gevangene", Self::FreePrisoner),
                    ("Verlaat het kamp", Self::WalkForward),
                ],
            },

            Self::SearchCamp => Question {
                text: "Je vindt lege voedselkratten, een beschadigde landkaart en een houten kist waarvan het slot recentelijk is opengebroken.\n\nOp de kaart is slechts één locatie omcirkeld: de oude ruïne.",
                choices: &[
                    ("Bevrijd de gevangene", Self::FreePrisoner),
                    ("Ga naar de ruïne", Self::RuinsEntrance),
                    ("Volg verse voetsporen", Self::AncientRoad),
                ],
            },

            Self::FreePrisoner => Question {
                text: "Zodra de touwen los zijn wrijft de oude man over zijn polsen.\n\n'Eindelijk... ik begon al te denken dat niemand nog nieuwsgierig genoeg was om vragen te stellen.'",
                choices: &[
                    ("Vraag wie hij is", Self::PrisonerStory),
                    ("Vraag wat hier gebeurd is", Self::PrisonerStory),
                    ("Loop weg", Self::IgnorePrisoner),
                ],
            },

            Self::PrisonerStory => Question {
                text: "De oude man glimlacht.\n\n'Iedereen vertelt hier hetzelfde verhaal, maar vanuit een ander gezichtspunt. Mensen noemen de elfen monsters. Elfen noemen de mensen indringers. Beiden hebben gelijk. Beiden liegen.'\n\nHij wijst naar het westen.\n\n'Zoek de ruïne. Daar begon alles.'",
                choices: &[
                    ("Ga naar de ruïne", Self::RuinsEntrance),
                    ("Zoek eerst de kluizenaar", Self::Hermit),
                    ("Je gelooft hem niet", Self::IgnorePrisoner),
                ],
            },

            Self::IgnorePrisoner => Question {
                text: "Je besluit dat je genoeg vreemde verhalen hebt gehoord. Terwijl je wegloopt roept de oude man je na.\n\n'Vergeet niet... degene die het hardst beweert de waarheid te spreken is meestal degene die het meeste te verliezen heeft.'",
                choices: &[
                    ("Ga naar de ruïne", Self::RuinsEntrance),
                    ("Loop verder over de oude weg", Self::AncientRoad),
                    (
                        "Denk na over wie hier eigenlijk liegt",
                        Self::TruthFragmentB,
                    ),
                ],
            },

            Self::AncientRoad => Question {
                text: "De begroeiing maakt plaats voor verweerde straatstenen. Blijkbaar liep hier ooit een echte weg. In de verte zie je rook opstijgen.",
                choices: &[
                    ("Onderzoek de rook", Self::Hermit),
                    ("Loop verder richting de ruïne", Self::RuinsEntrance),
                    ("Ga terug naar het bos", Self::WalkForward),
                ],
            },

            Self::OldBridge => Question {
                text: "De stenen brug kraakt vervaarlijk maar houdt stand. Midden op de brug ligt een glanzende munt. Wanneer je hem wilt oppakken hoor je achter je voetstappen.",
                choices: &[
                    ("Pak de munt", Self::MeetHunter),
                    ("Laat de munt liggen", Self::RiverBank),
                    ("Ren door", Self::RuinsEntrance),
                ],
            },

            Self::RiverBank => Question {
                text: "Langs de rivier zie je afdrukken van blote voeten. Niet menselijk, niet dierlijk. Ze verdwijnen abrupt in het water alsof de eigenaar simpelweg is opgelost.",
                choices: &[
                    ("Volg de rivier", Self::Hermit),
                    ("Keer terug naar de brug", Self::OldBridge),
                    ("Ga richting de ruïne", Self::RuinsEntrance),
                ],
            },

            Self::Hermit => Question {
                text: "Een kleine hut staat verscholen tussen eeuwenoude eiken. Voor de deur zit een grijsaard thee te drinken alsof oorlog en samenzweringen hem volledig ontgaan.",
                choices: &[
                    ("Maak een praatje", Self::TalkHermit),
                    ("Val hem aan", Self::AttackHermit),
                    ("Loop door", Self::RuinsEntrance),
                ],
            },

            Self::TalkHermit => Question {
                text: "De oude man schenkt zonder iets te vragen een tweede beker thee in.\n\n'Je bent vandaag al de vierde die denkt de waarheid te zoeken,' zegt hij lachend. 'Geen van jullie weet zelfs welke vraag gesteld moet worden.'",
                choices: &[
                    ("Vraag naar de ruïne", Self::RuinsEntrance),
                    ("Vraag naar de elfen", Self::PrisonerStory),
                    ("Bedank hem en vertrek", Self::AncientRoad),
                ],
            },

            Self::AttackHermit => Question {
                text: "Je trekt je wapen. Nog voordat je een stap kunt zetten tikt de oude man met zijn wandelstok tegen de grond. Plotseling sta je weer aan het begin van het bospad alsof de afgelopen uren nooit hebben plaatsgevonden.",
                choices: &[
                    ("Probeer het opnieuw", Self::Start),
                    ("Accepteer het vreemde lot", Self::WalkForward),
                ],
            },

            Self::RuinsEntrance => Question {
                text: "Voor je rijzen de overblijfselen op van een enorme stenen vesting. De hoofdpoort staat wagenwijd open. Links ontdek je achter klimop een smalle spleet die diep de grond in lijkt te lopen.",
                choices: &[
                    ("Ga door de hoofdpoort", Self::FrontGate),
                    ("Neem de geheime doorgang", Self::SecretPassage),
                    ("Keer terug", Self::AncientRoad),
                ],
            },
            Self::FrontGate => Question {
                text: "De grote hal is verrassend intact. Zonlicht valt door een ingestort dak op een cirkel van verweerde standbeelden. In het midden staat een stenen deur zonder zichtbaar slot.",
                choices: &[
                    ("Onderzoek de beelden", Self::MuralRoom),
                    ("Zoek een andere doorgang", Self::HiddenLibrary),
                    ("Open de stenen deur", Self::VaultHall),
                ],
            },

            Self::SecretPassage => Question {
                text: "De smalle tunnel daalt tientallen meters af. Uiteindelijk mondt hij uit in een gigantische ondergrondse grot waarin een stil meer ligt. Aan de overkant brandt een eenzame fakkel.",
                choices: &[
                    ("Zoek een boot", Self::BoatCrossing),
                    ("Loop langs de oever", Self::DeepTunnel),
                    ("Ga terug", Self::RuinsEntrance),
                ],
            },

            Self::VaultHall => Question {
                text: "Wanneer de deur opent beweegt de lucht alsof eeuwenoude longen eindelijk weer ademhalen. Midden in de ruimte staat een metalen constructie die zacht lijkt te zoemen.",
                choices: &[
                    ("Onderzoek de machine", Self::AncientMachine),
                    ("Lees de inscripties", Self::MuralRoom),
                    ("Verlaat de zaal", Self::FrontGate),
                ],
            },

            Self::MuralRoom => Question {
                text: "De muren tonen dezelfde gebeurtenis vanuit verschillende perspectieven. Op één afbeelding vallen mensen de elfen aan. Op de volgende lijken juist de elfen de agressor. Geen enkel reliëf spreekt het vorige volledig tegen.",
                choices: &[
                    ("Bestudeer alles zorgvuldig", Self::HiddenLibrary),
                    ("Negeer de afbeeldingen", Self::VaultHall),
                    ("Zoek naar een verklaring", Self::GhostGuardian),
                ],
            },

            Self::HiddenLibrary => Question {
                text: "Tussen ingestorte boekenkasten ligt een vrijwel onbeschadigd manuscript.\n\n'Geschiedenis is niets meer dan de versie van het verhaal die lang genoeg heeft overleefd.'",
                choices: &[
                    ("Lees verder", Self::GhostGuardian),
                    ("Neem het boek mee", Self::AncientMachine),
                    ("Ga terug", Self::FrontGate),
                ],
            },

            Self::AncientMachine => Question {
                text: "De machine reageert op je aanwezigheid.\n\nNiet met geluid, maar met keuzevoorstellen die je nog niet hebt gemaakt.\n\nAlsof hij jouw toekomstige beslissingen al kent.",
                choices: &[
                    ("Betreed interface", Self::CrownCore),
                    ("Onderbreek systeem", Self::SystemBreak),
                    ("Stap terug", Self::VaultHall),
                ],
            },

            Self::GhostGuardian => Question {
                text: "Langzaam verschijnt een doorschijnende figuur. Geen elf. Geen mens.\n\n'Welkom, reiziger,' zegt hij. 'Je bent al de zoveelste die denkt dat dit conflict over goed en kwaad gaat.'",
                choices: &[
                    ("Luister", Self::AnswerGuardian),
                    ("Val hem aan", Self::FightGuardian),
                    ("Vraag wie hij is", Self::AnswerGuardian),
                ],
            },

            Self::AnswerGuardian => Question {
                text: "Ik ben de laatste bewaker van de Kroon van Herinnering.\n\n'Wie haar draagt, kent de waarheid. Maar niemand overleeft zonder te veranderen.'",
                choices: &[
                    ("Waar is de kroon?", Self::UndergroundLake),
                    ("Ik hoef de waarheid niet te weten", Self::EscapeForest),
                    ("Waarom bewaak jij haar?", Self::UndergroundLake),
                ],
            },

            Self::FightGuardian => Question {
                text: "Je wapen gaat dwars door de geest heen. De bewaker zucht diep.\n\n'Geweld lost verbazingwekkend weinig metafysische problemen op.'\n\nDaarna tikt hij zacht tegen je voorhoofd.",
                choices: &[("...", Self::EndingDeath)],
            },

            Self::UndergroundLake => Question {
                text: "Onder de ruïne strekt zich een enorm zwart meer uit. Midden op het water rust een klein eiland waarop een stenen troon zichtbaar is.",
                choices: &[
                    ("Gebruik een boot", Self::BoatCrossing),
                    ("Zoek een andere route", Self::DeepTunnel),
                    ("Keer terug", Self::GhostGuardian),
                ],
            },

            Self::BoatCrossing => Question {
                text: "De houten boot beweegt zonder dat je hoeft te roeien. Halverwege het meer zie je onder het water tientallen gezichten omhoog kijken. Geen van hen beweegt.",
                choices: &[
                    ("Kijk naar beneden", Self::TakeCrown),
                    (
                        "Volg een licht dat onder het water gloeit",
                        Self::WarSimulationCore,
                    ),
                ],
            },

            Self::DeepTunnel => Question {
                text: "Een natuurlijke tunnel leidt uiteindelijk naar hetzelfde eiland, maar onderweg ontdek je verse voetafdrukken. Je bent hier duidelijk niet alleen.",
                choices: &[
                    ("Volg de sporen", Self::ElfCouncil),
                    ("Loop naar het eiland", Self::TakeCrown),
                    ("Onderzoek een zijkamer in de rotsen", Self::HiddenWarRoom),
                ],
            },

            Self::ElfCouncil => Question {
                text: "De elfen staan opnieuw voor je.\n\nMaar deze keer lijken ze niet verbaasd je te zien.\nAlsof je nooit bent weggeweest.",
                choices: &[
                    ("Luister", Self::ChooseSide),
                    ("Ren door", Self::TakeCrown),
                    ("Ga terug", Self::WalkForward),
                ],
            },

            Self::HumanCaptain => Question {
                text: "De kapitein lijkt je niet te herkennen als individu.\n\nAlleen als rol in een conflictstructuur.",
                choices: &[
                    ("Sluit je aan", Self::ChooseSide),
                    ("Weiger rol", Self::EndingReturn),
                    ("Zoek waarheid", Self::TrueKing),
                ],
            },

            Self::ChooseSide => Question {
                text: "Beide groepen kijken je zwijgend aan. Opmerkelijk genoeg lijken ze banger voor jouw beslissing dan voor elkaar.",
                choices: &[
                    ("Kies de elfen", Self::BecomeGuardian),
                    ("Kies de mensen", Self::TakeCrown),
                    ("Kies niemand", Self::LeaveWithTruth),
                ],
            },

            Self::EscapeForest => Question {
                text: "Je besluit dat sommige waarheden beter verborgen kunnen blijven. Zonder nog één keer achterom te kijken verlaat je het bos. De vogels beginnen weer te zingen.",
                choices: &[("Verder...", Self::EndingEscape)],
            },

            Self::BecomeGuardian => Question {
                text: "De oude geest glimlacht.\n\n'Dan begrijp je eindelijk dat macht niet bedoeld is om te bezitten.' Hij legt een hand op je schouder.",
                choices: &[("Aanvaard de taak", Self::EndingGuardian)],
            },

            Self::TakeCrown => Question {
                text: "Je zet de eeuwenoude kroon op. In één ogenblik beleef je duizenden jaren geschiedenis alsof je er zelf bij was. Plotseling begrijp je waarom iedereen slechts een deel van de waarheid kende.",
                choices: &[
                    ("Behoud de kroon", Self::EndingKing),
                    ("Vernietig de kroon", Self::DestroyCrown),
                ],
            },

            Self::DestroyCrown => Question {
                text: "Met alle kracht die je bezit sla je de kroon kapot. Een verblindende lichtflits vult de grot. Wanneer je weer kunt zien zijn zowel de geesten als de eeuwenoude spanningen verdwenen.",
                choices: &[("Kijk wat er gebeurt", Self::EndingPeace)],
            },

            Self::LeaveWithTruth => Question {
                text: "Je weigert partij te kiezen. In plaats daarvan schrijf je alles op wat je hebt gezien. Misschien zullen toekomstige generaties eindelijk begrijpen hoe oorlog werkelijk ontstaat.",
                choices: &[("Verlaat het bos", Self::Finish)],
            },
            Self::Finish => Question {
                text: "Je hebt het einde van dit avontuur bereikt.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },
            Self::EndingEscape => Question {
                text: "Je bereikt de bewoonde wereld. Niemand gelooft je verhaal over ruïnes, geesten en kronen. Langzaam begin je zelf te twijfelen of het allemaal echt gebeurd is.\n\nEinde: De Getuige.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingGuardian => Question {
                text: "De eeuwen glijden voorbij. Reizigers zien soms een eenzame wachter tussen de bomen, maar niemand spreekt hem ooit aan. Oorlogen komen en gaan. Het geheim blijft veilig.\n\nEinde: De Laatste Bewaker.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingKing => Question {
                text: "Met alle herinneringen van duizenden generaties in je hoofd word je de enige die de volledige waarheid kent. Helaas begrijpt niemand anders je nog.\n\nEinde: Koning Zonder Volk.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingPeace => Question {
                text: "Zonder de kroon verdwijnt ook de reden om erom te vechten. Elfen en mensen kijken elkaar zwijgend aan. Voor het eerst in eeuwen weet niemand wie er gewonnen heeft.\n\nMisschien is dat precies de bedoeling.\n\nEinde: De Laatste Oorlog.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingDeath => Question {
                text: "Wanneer je weer wakker wordt lig je aan de rand van het bos. Je herinnert je niets meer behalve een vaag gevoel dat geweld soms de snelste weg is naar onwetendheid.\n\nEinde: Vergeten.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::MemoryEcho => Question {
                text: "De scène herhaalt zich niet alleen — hij herstructureert zich.\n\nJe ziet jezelf keuzes maken die je nog niet hebt gemaakt.\n\nDe elf kijkt je aan alsof hij weet wat er daarna komt.",
                choices: &[
                    ("Onderzoek systeemgevoel", Self::CrownCore),
                    ("Volg herhaling", Self::TimeLoopStart),
                    ("Breek patroon", Self::SystemBreak),
                ],
            },

            Self::BrokenTimeHall => Question {
                text: "De ruimte is niet kapot.\nHij is herhaald.\n\nJe ziet dezelfde hal meerdere keren, licht verschoven in betekenis in plaats van ruimte.",
                choices: &[
                    ("Zoek consistentie", Self::TimeLoopStart),
                    ("Doorbreek structuur", Self::SystemBreak),
                    ("Volg afwijking", Self::MirrorElf),
                ],
            },

            Self::MirrorElf => Question {
                text: "De elf kijkt je aan, maar zijn gezicht synchroniseert niet meer met zijn bewegingen.\n\nJe ziet jezelf in hem, maar niet zoals je bent — zoals je zou kunnen zijn in een andere interpretatie van dezelfde scène.",
                choices: &[
                    ("Confronteer spiegel", Self::MirrorHuman),
                    ("Breek illusie", Self::CollapseChoice),
                    ("Accepteer rol", Self::ChooseSide),
                ],
            },

            Self::MirrorHuman => Question {
                text: "De menselijke kapitein en de elf staan naast elkaar.\nNiet langer vijanden, maar varianten van dezelfde figuur.\n\nDe verschillen lijken pas achteraf gekozen.",
                choices: &[
                    ("Onderzoek overeenkomst", Self::TruthFragmentC),
                    ("Ontken patroon", Self::CollapseChoice),
                    ("Kies zijde", Self::ChooseSide),
                ],
            },

            Self::HiddenWarRoom => Question {
                text: "Onder de ruïne opent zich een kamer vol glazen cilinders. In elke cilinder zie je een variant van dezelfde oorlog, steeds net iets anders verlopen.\n\nSommige eindigen in vrede. Andere in totale vernietiging. Eén eindigt nooit.",
                choices: &[
                    ("Breek de cilinders", Self::CollapseChoice),
                    ("Bestudeer ze", Self::TruthFragmentC),
                    ("Ga naar de kroon", Self::TakeCrown),
                ],
            },

            Self::WarSimulationCore => Question {
                text: "In het midden van de kamer staat een pulserend object. Geen machine, geen hart, maar iets ertussenin.\n\n'Conflict is de enige stabiele vorm die jullie begrijpen,' zegt de stem.",
                choices: &[
                    ("Schakel het uit", Self::CollapseChoice),
                    ("Communiceer ermee", Self::TruthFragmentC),
                    ("Aanvaard het", Self::EndingSimulation),
                ],
            },

            Self::TruthFragmentA => Question {
                text: "Je herinnert je plots fragmenten die niet in jouw verhaal passen. Je stond ooit niet aan de rand van het bos, maar erboven. Kijkend. Interpreterend.\n\nMaar het gevoel verdwijnt zodra je het probeert vast te houden.",
                choices: &[
                    ("Zoek meer herinneringen", Self::ThePlayerMemory),
                    ("Negeer het", Self::TakeCrown),
                ],
            },

            Self::TruthFragmentB => Question {
                text: "De woorden blijven hangen: 'Je hebt ons altijd tegenover elkaar gezet.'\n\nDe kapitein en de elf staan nu naast elkaar. Niet vijandig. Niet vriendelijk. Functioneel.",
                choices: &[
                    ("Wat ben ik dan?", Self::ThePlayerMemory),
                    ("Dwing een keuze af", Self::CollapseChoice),
                ],
            },

            Self::TruthFragmentC => Question {
                text: "De waarheid is niet één ding. Het is een verzameling stabiele interpretaties.\n\nElke versie van de wereld is consistent. Maar ze sluiten elkaar uit.",
                choices: &[
                    ("Accepteer meerdere waarheden", Self::EndingObserver),
                    ("Zoek één echte waarheid", Self::FalseKing),
                ],
            },

            Self::ThePlayerMemory => Question {
                text: "Je probeert jezelf te herinneren vóór het bos, maar daar is geen begin. Alleen rollen. Jager. Getuige. Ontwerper. De speler?\n\nDe grenzen beginnen te vervagen.",
                choices: &[
                    ("Herstructureer alles", Self::RebuildChoice),
                    ("Laat het instorten", Self::CollapseChoice),
                ],
            },

            Self::CollapseChoice => Question {
                text: "De wereld reageert niet met weerstand, maar met stilte. Alsof je een systeem hebt onderbroken dat zichzelf al niet meer volledig draaide.",
                choices: &[
                    ("Laat alles instorten", Self::EndingErase),
                    ("Probeer te stabiliseren", Self::RebuildChoice),
                ],
            },

            Self::RebuildChoice => Question {
                text: "Je probeert de fragmenten opnieuw te ordenen. Voor een moment ontstaat er een stabiele realiteit. Dan begint hij opnieuw te splitsen.",
                choices: &[
                    ("Volg één lijn", Self::FalseKing),
                    ("Blijf observeren", Self::EndingSplitSelf),
                ],
            },

            Self::FalseKing => Question {
                text: "Je kiest één versie van de waarheid en duwt de rest weg. De wereld wordt coherent, maar voelbaar smaller.\n\nIets in jou weet dat dit niet de echte oplossing was.",
                choices: &[
                    ("Claim de kroon opnieuw", Self::TakeCrown),
                    ("Laat los", Self::EndingBecoming),
                ],
            },

            Self::EndingSimulation => Question {
                text: "De oorlog stopt niet. Hij stabiliseert. Generaties leven en sterven binnen een gecontroleerde spanning die nooit escaleert.\n\nEinde: De Perfecte Cyclus.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingObserver => Question {
                text: "Je kiest ervoor om alle versies van de waarheid naast elkaar te laten bestaan. Je wordt geen deelnemer meer, maar een referentiepunt.\n\nEinde: De Getuige Buiten Tijd.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingErase => Question {
                text: "Wanneer alles verdwijnt blijft er geen overwinning of nederlaag over. Alleen stilte die geen contrast meer heeft.\n\nEinde: Null.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingSplitSelf => Question {
                text: "Je bestaanslijn vertakt zich oneindig. Elke keuze die je niet maakt, wordt alsnog geleefd door een ander jij.\n\nEinde: Veelvoud.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingBecoming => Question {
                text: "Je stopt met proberen de waarheid te bevatten en wordt onderdeel van de structuur zelf. Niet bewust. Niet onbewust. Iets ertussenin.\n\nEinde: Wording.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },
            Self::LoopMemory => Question {
                text: "Je blijft staan. Het bos verandert niet. Maar de stilte wel.\n\n'Je hebt deze keuze al vaker gemaakt,' lijkt iets te zeggen zonder woorden.",
                choices: &[
                    ("Herinner vorige loops", Self::TrueStart),
                    ("Negeer het gevoel", Self::SneakToElf),
                    ("Stap achteruit", Self::FalseStart),
                ],
            },
            Self::FalseStart => Question {
                text: "Wanneer je achteruit stapt merk je dat het pad achter je ontbreekt.\nNiet verdwenen. Nooit bestaan.\n\nDe wereld corrigeert zichzelf zonder je toestemming.",
                choices: &[
                    ("Accepteer correctie", Self::CrownInterface),
                    ("Weiger", Self::SystemBreak),
                    ("Zoek naar een niet-gecorrigeerde versie", Self::CrownShard),
                ],
            },
            Self::CrownCore => Question {
                text: "De kroon bestaat niet als object.\nAlleen als stabilisatie van tegenstrijdige interpretaties.\n\nJe staat nu binnen die stabilisatie.",
                choices: &[
                    ("Luister naar systeem", Self::SystemVoice),
                    ("Weiger interpretatie", Self::SystemBreak),
                    ("Zoek oorsprong", Self::TrueKing),
                ],
            },

            Self::SystemVoice => Question {
                text: "\"Je bent geen deelnemer.\" zegt de stem.\n\"Je bent de consistentie tussen conflicterende verhalen.\"\n\n\"Zonder jou divergeert alles.\"",
                choices: &[
                    ("Wat gebeurt er zonder mij?", Self::EndingNoExit),
                    ("Beïnvloed verhalen", Self::HumanCaptain),
                    ("Zoek centrum", Self::TrueKing),
                ],
            },

            Self::SystemBreak => Question {
                text: "De structuur van keuzes valt weg.\nNiet chaos.\nMaar potentieel zonder selectie.\n\nEr is geen pad meer om te volgen.",
                choices: &[
                    ("Laat los", Self::EndingPlayerErase),
                    ("Probeer opnieuw te structureren", Self::SystemRewrite),
                ],
            },
            Self::SystemRewrite => Question {
                text: "De wereld herstructureert zich.\nElfen worden mensen die ooit elfen waren.\nGeschiedenis herschrijft zichzelf terwijl je kijkt.\n\nEn jij blijft als referentiepunt bestaan.",
                choices: &[
                    ("Observeer", Self::EndingSystemMerge),
                    ("Stap eruit", Self::EndingFreedom),
                    ("Laat de tegenstrijdigheden bestaan", Self::EndingLoopBreak),
                ],
            },
            Self::CrownShard => Question {
                text: "De kroon breekt niet.\nHij vermenigvuldigt zich in interpretaties.\n\nElke versie toont een andere stabiele wereld die zichzelf gelooft.",
                choices: &[
                    ("Kies één realiteit", Self::FalseKing),
                    ("Accepteer multipliciteit", Self::EndingLoop),
                    ("Weiger selectie", Self::SystemBreak),
                ],
            },

            Self::EndingTrueStart => Question {
                text: "Je realiseert je dat er nooit een begin was.\nAlle verhalen zijn variaties op latere interpretaties.\n\nEinde: Geen oorsprong.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingLoopBreak => Question {
                text: "De realiteit stopt met proberen consistent te zijn.\nTegenstrijdigheden bestaan naast elkaar zonder conflict.\n\nEinde: Gelijktijdigheid.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingSystemMerge => Question {
                text: "Je wordt niet de gebruiker van het systeem.\nJe wordt de structuur waarin keuzes überhaupt mogelijk zijn.\n\nEinde: Integratie.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingPlayerErase => Question {
                text: "Er is geen jij meer die observeert.\nAlle keuzes bestaan nog, maar zonder centrum.\n\nEinde: Decentraal bestaan.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingFreedom => Question {
                text: "Voor het eerst valt het systeem niet uiteen en niet samen.\nHet laat je gewoon gaan.\n\nEinde: Ongebonden.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },
            Self::EndingReturn => Question {
                text: "Je keert terug naar een beginpunt dat jou niet meer volledig herkent.\n\nDe wereld is consistent — maar niet meer met jouw herinneringen.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },

            Self::EndingNoExit => Question {
                text: "Elke keuze leidt naar een andere versie van hetzelfde moment.\nJe hebt nooit een uitgang gehad, alleen varianten ervan.\n\nEinde: Gesloten systeem.",
                choices: &[("Opnieuw spelen", Self::Start)],
            },
            Self::KillElf => Question {
                text: "Je steekt toe.\n\nMaar dit keer gebeurt er iets vreemds: de elf sterft niet meteen.\nHij kijkt je aan alsof hij je herkent.\n\n'Je hebt dit eerder gedaan,' zegt hij zacht.",
                choices: &[
                    ("Doorzetten", Self::LootElf),
                    ("Terugdeinzen", Self::WaitForElf),
                    ("Weglopen", Self::TurnAround),
                ],
            },
            Self::SneakToElf => Question {
                text: "Je verstopt je langs het pad.\n\nDe elf spreekt met iemand die je niet kunt zien.\nMaar je hoort twee gesprekken tegelijk, net niet synchroon.\n\nAlsof iemand de scène opnieuw afspeelt met kleine afwijkingen.",
                choices: &[
                    ("Aanvallen", Self::KillElf),
                    ("Blijven observeren", Self::KeepListening),
                    ("Terugtrekken", Self::WalkForward),
                ],
            },
            Self::WaitForElf => Question {
                text: "Je wacht.\n\nDe elf draait zich om.\nMaar in dezelfde beweging draait hij zich ook niet om.\nTwee versies van hetzelfde moment overlappen elkaar.\n\nEén van hen ziet jou.",
                choices: &[
                    ("Benader hem", Self::KillElf),
                    ("Verbergen", Self::SneakToElf),
                    ("Weglopen", Self::TurnAround),
                ],
            },
            Self::TrueKing => Question {
                text: "Er is geen troon.\nAlleen een positie die telkens opnieuw wordt ingevuld door iets dat stabiliteit vereist.",
                choices: &[
                    ("Neem positie", Self::CrownCore),
                    ("Weiger positie", Self::EndingTrueStart),
                    ("Breek cyclus", Self::TimeLoopStart),
                ],
            },

            Self::TimeLoopStart => Question {
                text: "Je herkent dit moment als een knooppunt in plaats van een begin.\n\nDe wereld wacht op afwijking.",
                choices: &[
                    ("Afwijken", Self::MemoryEcho),
                    ("Herhalen", Self::SneakToElf),
                    ("Stop tijdlijn", Self::TimeLoopEnd),
                ],
            },

            Self::TimeLoopEnd => Question {
                text: "De lus stabiliseert opnieuw.\nMaar niet exact hetzelfde.\n\nEr blijft een restverschil bestaan.",
                choices: &[
                    ("Terug naar begin", Self::Start),
                    ("Volg afwijking", Self::EndingReturn),
                    ("Onderzoek het restverschil", Self::BrokenTimeHall),
                ],
            },

            Self::EndingLoop => Question {
                text: "De lus sluit zich niet.\nHij verdicht zich.\n\nAlle keuzes die je hebt gemaakt blijven bestaan, maar verliezen hun volgorde.\n\nJe beseft dat “volgende” en “vorige” hier geen betekenis meer hebben.",
                choices: &[
                    ("Begin opnieuw", Self::Start),
                    ("Observeer structuur", Self::TrueStart),
                ],
            },

            Self::TrueStart => Question {
                text: "Je zoekt naar het begin.\n\nMaar elk begin dat je vindt blijkt een middenpunt van iets anders te zijn.\n\nEr is geen oorsprong, alleen instapmomenten in een systeem dat al draaide.",
                choices: &[
                    ("Ga het bos in", Self::SneakToElf),
                    ("Onderzoek de lus", Self::EndingLoop),
                    ("Zoek de kroon", Self::CrownInterface),
                ],
            },

            Self::CrownInterface => Question {
                text: "Je staat niet langer vóór de kroon.\nJe staat ín de functie ervan.\n\nHet object is verdwenen. Wat overblijft is een mechanisme dat keuzes normaliseert tot verhalen.",
                choices: &[
                    ("Neem controle", Self::SystemRewrite),
                    ("Breek structuur", Self::SystemBreak),
                    ("Stap terug", Self::TrueStart),
                ],
            },
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H"); // Clear the screen and put cursor at (1,1)
}

fn print_screen(question: &Question) {
    clear_screen();
    typewrite(&question.to_string());
    print!("> ");
    std::io::stdout().flush().expect("Unable to flush stdout");
}

fn typewrite(text: &str) {
    let mut stdout = std::io::stdout();
    for c in text.chars() {
        print!("{}", c);
        stdout.flush().expect("Unable to flush stdout");
        let millis = match c {
            '.' | '\n' => 500,
            ',' => 100,
            ' ' => 50,
            _ => 40,
        };
        std::thread::sleep(Duration::from_millis(millis));
    }
}

fn main() {
    clear_screen();
    typewrite(
        "Welkom bij de \"Build your own adventure game\".\nJe kunt ten aller tijde '0' typen om het spel af te sluiten.\nDruk op Enter om het spel te starten...",
    );
    {
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        let _ = stdin
            .read_line(&mut buffer)
            .expect("Couldn't read from stdin");
    }
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
