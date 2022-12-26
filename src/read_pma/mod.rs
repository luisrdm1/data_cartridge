use std::{collections::HashMap, io::Read, str, str::FromStr};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PMAFile(pub Vec<PMANode>);

impl PMAFile {
    pub fn read(&mut self, content: &str) {
        // Current node which is being worked on
        let mut current_node = PMANode::default();

        // Current child object, just for some cases
        let mut current_child = IObjeto::default();

        // Indicates if working inside sons
        let mut inside_sons = false;

        // Indicates where to work on the current node
        let mut current_working_place = NodeType::Other;

        for line in content.lines() {
            // Is node ?
            let is_node = is_node(line);

            // Level of indentation of the current line (by number of '\t')
            let actual_level = get_indentation_level(line);

            if actual_level <= current_child.level
                && !current_child.common_data.is_empty()
                && inside_sons
            {
                current_node.sons.push(current_child);
                current_child = IObjeto::default();
            } else if actual_level <= current_node.level && !current_node.name.is_empty() {
                self.push_nod(current_node);
                current_node = PMANode::default();
                inside_sons = false;
            }

            // It's a node
            if is_node && !line.contains("Nil") {
                // Splits the line, if it contains a '-'
                let splitted_line = line.trim().split('-').collect::<Vec<&str>>();
                // Sets the working place
                current_working_place = splitted_line[0].trim().parse().unwrap();
                match current_working_place {
                    NodeType::IObjeto => {
                        if inside_sons {
                            current_child.update_level(actual_level);
                        } else {
                            // Sets the name
                            current_node.update_name(line);
                            // Sets the node level
                            current_node.update_level(actual_level);
                        }
                    }
                    NodeType::Filhos => {
                        if actual_level == current_node.level + 1 {
                            inside_sons = true;
                        }
                    }
                    NodeType::Other => {
                        // Sets the name
                        current_node.update_name(line);
                        // Sets the node level
                        current_node.update_level(actual_level);
                    }
                    _ => (),
                }
            } else {
                // It's a parameter
                let (k, v) = split_into_tuple(line, "=");
                match current_working_place {
                    NodeType::DadosComuns => {
                        if inside_sons {
                            current_child.common_data.insert(k, v);
                        } else {
                            current_node.object_data.common_data.insert(k, v);
                        }
                    }
                    NodeType::DadosEspecificos => {
                        if inside_sons {
                            current_child.specific_data.0.insert(k, v);
                        } else {
                            current_node.object_data.specific_data.0.insert(k, v);
                        }
                    }
                    NodeType::GateIn => {
                        if inside_sons {
                            current_child
                                .specific_data
                                .1
                                .gate_in
                                .parameters
                                .insert(k, v);
                        } else {
                            current_node
                                .object_data
                                .specific_data
                                .1
                                .gate_in
                                .parameters
                                .insert(k, v);
                        }
                    }
                    NodeType::GateOut => {
                        if inside_sons {
                            current_child
                                .specific_data
                                .1
                                .gate_out
                                .parameters
                                .insert(k, v);
                            if current_child
                                .specific_data
                                .1
                                .gate_out
                                .parameters
                                .get("StringConfiguracao")
                                .is_some()
                            {
                                current_working_place = NodeType::DadosEspecificos;
                            }
                        } else {
                            current_node
                                .object_data
                                .specific_data
                                .1
                                .gate_out
                                .parameters
                                .insert(k, v);
                            if current_node
                                .object_data
                                .specific_data
                                .1
                                .gate_out
                                .parameters
                                .get("StringConfiguracao")
                                .is_some()
                            {
                                current_working_place = NodeType::DadosEspecificos;
                            }
                        }
                    }
                    NodeType::Other => current_node.add_parameter(k, v),
                    _ => {}
                }
            }
        }
        if !current_child.common_data.is_empty() {
            current_node.sons.push(current_child);
        }
        self.push_nod(current_node);
    }

    fn push_nod(&mut self, node: PMANode) {
        self.0.push(node)
    }
}

type Param = HashMap<String, Option<String>>;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PMANode {
    pub level: usize,
    name: String,
    parameters: Param,
    object_data: IObjeto,
    sons: Vec<IObjeto>,
}

impl PMANode {
    pub fn add_parameter(&mut self, k: String, v: Option<String>) {
        self.parameters.insert(k, v);
    }

    pub fn update_name(&mut self, name: &str) {
        self.name = name.trim().to_owned();
    }

    pub fn update_level(&mut self, level: usize) {
        self.level = level;
    }

    /// Implements a search inside the parent Node, by type, returning the Node with all of its subnodes
    pub fn get_nodes_by_type(&self, tipo: &str) -> Option<Self> {
        match self.parameters.get(tipo) {
            Some(_) => Some(self).cloned(),
            _ => None,
        }
    }
}

fn split_into_tuple(string: &str, separator: &str) -> (String, Option<String>) {
    let words = string
        .split(separator)
        .map(|s| s.trim().to_owned())
        .collect::<Vec<_>>();

    (words[0].clone(), words.get(1).cloned())
}

#[derive(Debug, Default)]
pub struct Rota(Vec<IObjeto>);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IObjeto {
    pub level: usize,
    // Dados comuns
    common_data: Param,
    // Dados específicos .0 to acces Param and .1 to IObjetoNav
    specific_data: (Param, IObjetoNav),
}

impl IObjeto {
    pub fn update_level(&mut self, level: usize) {
        self.level = level;
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IObjetoNav {
    gate_in: Gate,
    gate_out: Gate,
}

impl IObjeto {}

#[derive(Debug, Default, Clone, PartialEq)]
struct Gate {
    parameters: Param,
}

impl Gate {}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum NodeType {
    #[default]
    IObjeto,
    IObjetoNav,
    GateIn,
    GateOut,
    DadosComuns,
    DadosEspecificos,
    Filhos,
    Other,
}

impl FromStr for NodeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IObjetoNav" => Ok(NodeType::IObjetoNav),
            "IObjeto" => Ok(NodeType::IObjeto),
            "GateIn" => Ok(NodeType::GateIn),
            "GateOut" => Ok(NodeType::GateOut),
            "DadosComuns" => Ok(NodeType::DadosComuns),
            "DadosEspecificos" => Ok(NodeType::DadosEspecificos),
            "Filhos" => Ok(NodeType::Filhos),
            _ => Ok(NodeType::Other),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
enum Nav {
    #[default]
    Rota,
    Waypoint,
    Reccelite,
    Perna,
}

impl FromStr for Nav {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NavPerna" => Ok(Nav::Perna),
            "NavReccelite" => Ok(Nav::Reccelite),
            "NavWaypoint" => Ok(Nav::Waypoint),
            "NavRota" => Ok(Nav::Rota),
            _ => Err(()),
        }
    }
}

pub fn read_file(path: &str) -> String {
    let file = std::fs::File::open(path).unwrap();
    let mut rdr = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::WINDOWS_1252))
        .build(file);

    let mut content = String::new();
    match rdr.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => eprintln!("Something went wrong: {e}"),
    }
    content
}

fn get_indentation_level(line: &str) -> usize {
    line.trim_end()
        .chars()
        .map(|c| c == '\t')
        .filter(|b| *b)
        .count()
}

fn is_node(line: &str) -> bool {
    // Has "-" or "=" in the line.
    let eq_index = line.find('=');
    let hyphen_index = line.find('-');

    // Is node ?
    line.contains('-') && !line.contains('=')
        || hyphen_index < eq_index && hyphen_index.is_some() && eq_index.is_some()
        || !line.contains('-') && !line.contains('=')
}

mod tests {
    use super::*;

    #[test]
    fn test_reading_pma_file() {
        let content = "\
IObjeto - Rota 2
	DadosComuns
		Nome = Rota 2
		Tipo = NavRota
	DadosEspecificos
		IObjetoNav
			GateIn
				Posicao.Lat = 0
				Posicao.Lon = 0
				StringConfiguracao = 6910 206 4572
			GateOut
				Posicao.Lat = 0
				Posicao.Lon = 0
				StringConfiguracao = 6910 206 4572
		AeronaveNome = A1_0A
	Filhos
		IObjeto - SM11
			DadosComuns
				Nome = SM11
				Tipo = NavWaypoint
			DadosEspecificos
				IObjetoNav
					GateIn
						Posicao.Lat = -29.710547222
						Posicao.Lon = -53.703902778
						StringConfiguracao = 6910 206 4572
					GateOut
						Posicao.Lat = -29.710547222
						Posicao.Lon = -53.703902778
						StringConfiguracao = 6910 206 4572
				WaypointSubtipo = 2694x45
			Filhos
";
        let mut file = PMAFile::default();
        file.read(&content);
        assert_eq!(
            file,
            PMAFile(vec![PMANode {
                level: 0,
                name: "IObjeto - Rota 2".to_owned(),
                parameters: HashMap::new(),
                object_data: IObjeto {
                    level: 0,
                    common_data: HashMap::from([
                        ("Nome".to_owned(), Some("Rota 2".to_owned())),
                        ("Tipo".to_owned(), Some("NavRota".to_owned()))
                    ]),
                    specific_data: (
                        HashMap::from([("AeronaveNome".to_owned(), Some("A1_0A".to_owned()))]),
                        IObjetoNav {
                            gate_in: Gate {
                                parameters: HashMap::from([
                                    ("Posicao.Lat".to_owned(), Some("0".to_owned())),
                                    ("Posicao.Lon".to_owned(), Some("0".to_owned())),
                                    (
                                        "StringConfiguracao".to_owned(),
                                        Some("6910 206 4572".to_owned())
                                    )
                                ])
                            },
                            gate_out: Gate {
                                parameters: HashMap::from([
                                    ("Posicao.Lat".to_owned(), Some("0".to_owned())),
                                    ("Posicao.Lon".to_owned(), Some("0".to_owned())),
                                    (
                                        "StringConfiguracao".to_owned(),
                                        Some("6910 206 4572".to_owned())
                                    )
                                ])
                            },
                        }
                    )
                },
                sons: vec![IObjeto {
                    level: 2,
                    common_data: HashMap::from([
                        ("Nome".to_owned(), Some("SM11".to_owned())),
                        ("Tipo".to_owned(), Some("NavWaypoint".to_owned()))
                    ]),
                    specific_data: (
                        HashMap::from([("WaypointSubtipo".to_owned(), Some("2694x45".to_owned()))]),
                        IObjetoNav {
                            gate_in: Gate {
                                parameters: HashMap::from([
                                    ("Posicao.Lat".to_owned(), Some("-29.710547222".to_owned())),
                                    ("Posicao.Lon".to_owned(), Some("-53.703902778".to_owned())),
                                    (
                                        "StringConfiguracao".to_owned(),
                                        Some("6910 206 4572".to_owned())
                                    )
                                ])
                            },
                            gate_out: Gate {
                                parameters: HashMap::from([
                                    ("Posicao.Lat".to_owned(), Some("-29.710547222".to_owned())),
                                    ("Posicao.Lon".to_owned(), Some("-53.703902778".to_owned())),
                                    (
                                        "StringConfiguracao".to_owned(),
                                        Some("6910 206 4572".to_owned())
                                    )
                                ])
                            }
                        }
                    ),
                }]
            }])
        )
    }
}
