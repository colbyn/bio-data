use std::collections::{HashMap, LinkedList, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    database_name: String,
    identifier: String,
}

impl Identifier {
    pub fn from_str(val: &str) -> Option<Self> {
        match &val.split(":").collect::<Vec<_>>()[..] {
            [database_name, identifier] => {
                Some(Identifier {
                    database_name: database_name.to_owned().to_owned(),
                    identifier: identifier.to_owned().to_owned(),
                })
            }
            _ => None
        }
    }
    pub fn is_gene(&self) -> bool {
        self.database_name.as_str() == "entrez gene/locuslink" ||
        self.database_name.as_str() == "ensembl" ||
        self.database_name.as_str() == "ensemblGenome"
    }
    pub fn is_nucleic_acid(&self) -> bool {
        unimplemented!()
    }
    /// Uses Chebi Identifiers
    pub fn is_small_molecule(&self) -> bool {
        unimplemented!()
    }
    /// Uses Chebi Identifiers
    pub fn is_protein(&self) -> bool {
        unimplemented!()
    }
}

/// Columns:
/// - `#ID Interactor A`
/// - `ID Interactor B`
/// - `Alt IDs Interactor A`
/// - `Alt IDs Interactor B`
/// - `Aliases Interactor A`
/// - `Aliases Interactor B`
/// - `Interaction Detection Method`
/// - `Publication 1st Author`
/// - `Publication Identifiers`
/// - `Taxid Interactor A`
/// - `Taxid Interactor B`
/// - `Interaction Types`
/// - `Source Database`
/// - `Interaction Identifiers`
/// - `Confidence Values`
#[derive(Debug, Clone)]
pub struct Row {
    /// `#ID Interactor A`
    id_interactor_a: Identifier,
    /// `ID Interactor B`
    id_interactor_b: Identifier,
    /// `Alt IDs Interactor A`
    alt_ids_interactor_a: HashSet<String>,
    /// `Alt IDs Interactor B`
    alt_ids_interactor_b: HashSet<String>,
    /// `Aliases Interactor A`
    aliases_interactor_a: HashSet<String>,
    /// `Aliases Interactor B`
    aliases_interactor_b: HashSet<String>,
    /// `Interaction Detection Method`
    interaction_detection_method: String,
    /// `Publication 1st Author`
    publication_1st_auth_r: String,
    /// `Publication Identifiers`
    publication_identifiers: HashSet<String>,
    /// `Taxid Interactor A`
    taxid_interactor_a: String,
    /// `Taxid Interactor B`
    taxid_interactor_b: String,
    /// `Interaction Types`
    interaction_types: HashSet<String>,
    /// `Source Database`
    source_database: String,
    /// `Interaction Identifiers`
    interaction_identifiers: HashSet<String>,
    /// `Confidence Values`
    confidence_values: HashSet<String>,
}


fn verify_header(row: &str) {
    let mut rows: LinkedList<&str> = row
        .split("\t")
        .collect::<LinkedList<_>>();
    assert_eq!(rows.pop_front().unwrap(), "#ID Interactor A");
    assert_eq!(rows.pop_front().unwrap(), "ID Interactor B");
    assert_eq!(rows.pop_front().unwrap(), "Alt IDs Interactor A");
    assert_eq!(rows.pop_front().unwrap(), "Alt IDs Interactor B");
    assert_eq!(rows.pop_front().unwrap(), "Aliases Interactor A");
    assert_eq!(rows.pop_front().unwrap(), "Aliases Interactor B");
    assert_eq!(rows.pop_front().unwrap(), "Interaction Detection Method");
    assert_eq!(rows.pop_front().unwrap(), "Publication 1st Author");
    assert_eq!(rows.pop_front().unwrap(), "Publication Identifiers");
    assert_eq!(rows.pop_front().unwrap(), "Taxid Interactor A");
    assert_eq!(rows.pop_front().unwrap(), "Taxid Interactor B");
    assert_eq!(rows.pop_front().unwrap(), "Interaction Types");
    assert_eq!(rows.pop_front().unwrap(), "Source Database");
    assert_eq!(rows.pop_front().unwrap(), "Interaction Identifiers");
    assert_eq!(rows.pop_front().unwrap(), "Confidence Values");
}

fn parse_row(row_text: &str) -> Row {
    let mut rows: LinkedList<&str> = row_text
        .split("\t")
        .collect::<LinkedList<_>>();
    let pop_list = |rows: &mut LinkedList<&str>| {
        rows.pop_front().unwrap().split("|").map(|x| x.to_owned()).collect()
    };
    let id_interactor_a = Identifier::from_str(rows.pop_front().unwrap()).unwrap();
    let id_interactor_b = Identifier::from_str(rows.pop_front().unwrap()).unwrap();
    let alt_ids_interactor_a = pop_list(&mut rows);
    let alt_ids_interactor_b = pop_list(&mut rows);
    let aliases_interactor_a = pop_list(&mut rows);
    let aliases_interactor_b = pop_list(&mut rows);
    let interaction_detection_method = rows.pop_front().unwrap().to_owned();
    let publication_1st_auth_r = rows.pop_front().unwrap().to_owned();
    let publication_identifiers = pop_list(&mut rows);
    let taxid_interactor_a = rows.pop_front().unwrap().to_owned();
    let taxid_interactor_b = rows.pop_front().unwrap().to_owned();
    let interaction_types = pop_list(&mut rows);
    let source_database = rows.pop_front().unwrap().to_owned();
    let interaction_identifiers = pop_list(&mut rows);
    let confidence_values = pop_list(&mut rows);
    Row{
        id_interactor_a,
        id_interactor_b,
        alt_ids_interactor_a,
        alt_ids_interactor_b,
        aliases_interactor_a,
        aliases_interactor_b,
        interaction_detection_method,
        publication_1st_auth_r,
        publication_identifiers,
        taxid_interactor_a,
        taxid_interactor_b,
        interaction_types,
        source_database,
        interaction_identifiers,
        confidence_values,
    }
}


pub fn parse_file(path: &str) -> Vec<Row> {
    let source = std::fs::read_to_string(path).unwrap();
    let mut lines = source.lines();
    verify_header(lines.next().unwrap());
    lines
        .map(|row| parse_row(row))
        .collect::<Vec<_>>()
}

pub fn parse_ident(ty: &str) -> Option<String> {
    match ty {
        // increases
        "psi-mi:\\\"MI:0794\\\"(synthetic genetic interaction defined by inequality)" => {Some(String::from("increases"))}
        "psi-mi:\\\"MI:0799\\\"(additive genetic interaction defined by inequality)" => {Some(String::from("increases"))}
        "psi-mi:\\\"MI:0796\\\"(suppressive genetic interaction defined by inequality)" => {Some(String::from("increases"))}
        // association
        "psi-mi:\\\"MI:0403\\\"(colocalization)" => {Some(String::from("association"))}
        "psi-mi:\\\"MI:0914\\\"(association)" => {Some(String::from("association"))}
        "psi-mi:\\\"MI:0915\\\"(physical association)" => {Some(String::from("association"))}
        // directlyIncreases
        "psi-mi:\\\"MI:0407\\\"(direct interaction)" => {Some(String::from("directlyIncreases"))}
        _ => None
    }
}

pub fn parse_interaction_type(ty: &str) -> Option<String> {
    match ty {
        // increases
        "psi-mi:\\\"MI:0794\\\"(synthetic genetic interaction defined by inequality)" => {Some(String::from("increases"))}
        "psi-mi:\\\"MI:0799\\\"(additive genetic interaction defined by inequality)" => {Some(String::from("increases"))}
        "psi-mi:\\\"MI:0796\\\"(suppressive genetic interaction defined by inequality)" => {Some(String::from("increases"))}
        // association
        "psi-mi:\\\"MI:0403\\\"(colocalization)" => {Some(String::from("association"))}
        "psi-mi:\\\"MI:0914\\\"(association)" => {Some(String::from("association"))}
        "psi-mi:\\\"MI:0915\\\"(physical association)" => {Some(String::from("association"))}
        // directlyIncreases
        "psi-mi:\\\"MI:0407\\\"(direct interaction)" => {Some(String::from("directlyIncreases"))}
        _ => None
    }
}


pub fn main() {
    let source_file = "source.txt";
    let source_file = "data/biogrid/BIOGRID-ALL-4.2.193.mitab.txt";
    let mut rows = parse_file(source_file);
    let mut idents = HashSet::new();
    for row in rows {
        idents.insert(row.id_interactor_a.database_name.clone());
        if row.id_interactor_a.database_name.as_str() == "biogrid" {
            println!("\t{:?}", row.id_interactor_a);
        }
    }
    println!("\n");
    println!("{:?}", idents);
}

