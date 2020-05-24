use oac_parser::spec::{Document, Pattern};

const NAVAL_FATE: &str = include_str!("fixtures/passing/naval_fate.json");

#[test]
fn parse_naval_fate() -> Result<(), Box<dyn std::error::Error>> {
    let document: Document = serde_json::from_str(NAVAL_FATE)?;
    assert_eq!("1.0", &document.openautocompletion.version);
    assert_eq!("naval_fate", &document.cli.name);
    match &document.cli.pattern_groups[0] {
        Pattern::Group { patterns, .. } => match &patterns[0] {
            Pattern::Command { command, .. } => match command.resolve(&document) {
                Ok(command) => assert_eq!("ship", &command.names[0]),
                Err(error) => return Err(Box::new(error)),
            },
            _ => unreachable!(r#"patterns[0] is not of type "command""#),
        },
        _ => unreachable!(r#"pattern_groups[0] is not of type "group""#),
    }
    let arguments = &document.components.arguments.unwrap();
    assert_eq!(4, arguments.len());
    Ok(())
}
