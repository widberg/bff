use std::collections::VecDeque;
use std::path::PathBuf;

use bff::names::{
    WORDLIST_ANIMALS,
    WORDLIST_BIP39,
    get_forced_hash_string,
    parse_forced_hash_name,
};
use bff::petgraph;
use bff::petgraph::visit::{VisitMap, Visitable};
use clap::ValueEnum;
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::BffCliResult;
use crate::extract::{read_bigfile, read_bigfile_names, read_in_names, write_names};

#[derive(ValueEnum, Clone, Copy)]
pub enum Wordlist {
    Empty,
    Animals,
    BIP39,
}

pub fn names(
    bigfile_path: &Option<PathBuf>,
    wordlist: &Option<Wordlist>,
    in_names: &Vec<PathBuf>,
    out_names: &Option<PathBuf>,
    use_reference_graph: &bool,
) -> BffCliResult<()> {
    if let Some(bigfile_path) = bigfile_path {
        read_bigfile_names(bigfile_path)?;
    }
    read_in_names(in_names)?;

    if let Some(bigfile_path) = bigfile_path {
        read_bigfile_names(bigfile_path)?;

        let bigfile = read_bigfile(bigfile_path, &None, &None)?;

        if let Some(wordlist) = wordlist {
            if *use_reference_graph {
                let progress_bar = ProgressBar::new_spinner();
                progress_bar.set_message("Generating reference graph");
                let graph = bigfile.reference_graph();

                progress_bar.set_message("Finding roots");
                let mut discovered = graph.visit_map();
                let mut stack = VecDeque::new();

                graph
                    .node_indices()
                    .filter(|&node| {
                        graph
                            .neighbors_directed(node, petgraph::Direction::Incoming)
                            .count()
                            == 0
                    })
                    .for_each(|node| {
                        discovered.visit(node);
                        stack.push_front((node, None));
                    });

                progress_bar.set_style(ProgressStyle::default_bar());
                progress_bar.set_length(graph.node_count() as u64);

                while let Some((node, parent)) = stack.pop_front() {
                    progress_bar.inc(1);

                    for succ in graph.neighbors(node) {
                        if discovered.visit(succ) {
                            stack.push_back((succ, Some(node)));
                        }
                    }

                    let name = graph.node_weight(node).unwrap();
                    let name_in_db = bff::names::names().lock().unwrap().get(name).is_some();
                    if !name_in_db {
                        let string = match wordlist {
                            Wordlist::Empty => "".to_owned(),
                            Wordlist::Animals => name.get_wordlist_encoded_string(WORDLIST_ANIMALS),
                            Wordlist::BIP39 => name.get_wordlist_encoded_string(WORDLIST_BIP39),
                        };
                        let class = if let Some(resource) = bigfile.resources.get(name) {
                            format!(".{}", resource.class_name)
                        } else {
                            "".to_owned()
                        };
                        let name_string = if let Some(parent) = parent {
                            let parent_name = graph.node_weight(parent).unwrap().to_string();
                            let parent_string =
                                if let Some((_, s)) = parse_forced_hash_name(&parent_name) {
                                    s
                                } else {
                                    parent_name
                                };
                            format!("{}>{}{}", parent_string, string, class)
                        } else {
                            format!("{}{}", string, class)
                        };
                        bff::names::names()
                            .lock()
                            .unwrap()
                            .insert(&get_forced_hash_string(name, name_string));
                    }
                }
            } else {
                for resource in bigfile.resources.values() {
                    let name = &resource.name;
                    let class = resource.class_name.to_string();
                    let mut names_db = bff::names::names().lock().unwrap();
                    if names_db.get(name).is_none() {
                        let string = match wordlist {
                            Wordlist::Empty => "".to_owned(),
                            Wordlist::Animals => name.get_wordlist_encoded_string(WORDLIST_ANIMALS),
                            Wordlist::BIP39 => name.get_wordlist_encoded_string(WORDLIST_BIP39),
                        };
                        names_db.insert(&get_forced_hash_string(
                            name,
                            format!("{}.{}", string, class),
                        ));
                    }
                }
            }
        }

        if let Some(out_names) = out_names {
            write_names(out_names, &Some(bigfile.resources.keys().collect()))?;
        }
    } else if let Some(out_names) = out_names {
        write_names(out_names, &None)?;
    }

    Ok(())
}
