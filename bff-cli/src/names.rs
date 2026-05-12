use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use bff::names::{
    NameContext,
    NameType,
    WORDLIST_ANIMALS,
    WORDLIST_BIP39,
    get_forced_hash_string_for_type,
};
use bff::petgraph;
use bff::petgraph::visit::{VisitMap, Visitable};
use clap::ValueEnum;
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::BffCliResult;
use crate::shared::{
    probe_bigfile_name_context,
    read_bigfile,
    read_bigfile_names,
    read_in_names,
    write_names,
};

#[derive(ValueEnum, Clone, Copy)]
pub enum Wordlist {
    Empty,
    Animals,
    BIP39,
}

pub fn names(
    bigfile_path: Option<&Path>,
    name_type: Option<NameType>,
    wordlist: Option<Wordlist>,
    in_names: &[PathBuf],
    out_names: Option<&Path>,
    use_reference_graph: bool,
) -> BffCliResult<()> {
    let mut name_context = if let Some(bigfile_path) = bigfile_path {
        probe_bigfile_name_context(bigfile_path, None, None)?
    } else {
        NameContext::new(name_type.ok_or_else(|| {
            std::io::Error::other("`--name-type` is required when `--bigfile` is not provided")
        })?)
    };
    if let Some(bigfile_path) = bigfile_path {
        read_bigfile_names(bigfile_path, &mut name_context)?;
    }
    read_in_names(in_names, &mut name_context)?;

    if let Some(bigfile_path) = bigfile_path {
        let bigfile = read_bigfile(bigfile_path, None, None, &name_context)?;

        if let Some(wordlist) = wordlist {
            if use_reference_graph {
                let progress_bar = ProgressBar::new_spinner();
                progress_bar.set_message("Generating reference graph");
                let graph = bigfile.reference_graph(&name_context);

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

                    let name = *graph.node_weight(node).unwrap();
                    let name_in_db = name_context.contains(name);
                    if !name_in_db {
                        let string = match wordlist {
                            Wordlist::Empty => "".to_owned(),
                            Wordlist::Animals => name
                                .with_context(&name_context)
                                .get_wordlist_encoded_string(WORDLIST_ANIMALS),
                            Wordlist::BIP39 => name
                                .with_context(&name_context)
                                .get_wordlist_encoded_string(WORDLIST_BIP39),
                        };
                        let class = if let Some(bff_resource) = bigfile.bff_resource(name) {
                            format!(
                                ".{}",
                                bff_resource.resource.class_name.with_context(&name_context)
                            )
                        } else {
                            "".to_owned()
                        };
                        let name_string = if let Some(parent) = parent {
                            let parent_name = graph
                                .node_weight(parent)
                                .unwrap()
                                .with_context(&name_context)
                                .to_string();
                            let parent_string = if let Some((_, s)) = name_context
                                .name_type()
                                .parse_forced_hash_name(&parent_name)
                            {
                                s
                            } else {
                                parent_name
                            };
                            format!("{}>{}{}", parent_string, string, class)
                        } else {
                            format!("{}{}", string, class)
                        };
                        name_context.insert(&get_forced_hash_string_for_type(
                            name_context.name_type(),
                            name,
                            name_string,
                        ));
                    }
                }
            } else {
                for bff_resource in bigfile.bff_resources() {
                    let name = bff_resource.resource.name;
                    let class = bff_resource
                        .resource
                        .class_name
                        .with_context(&name_context)
                        .to_string();
                    if !name_context.contains(name) {
                        let string = match wordlist {
                            Wordlist::Empty => "".to_owned(),
                            Wordlist::Animals => name
                                .with_context(&name_context)
                                .get_wordlist_encoded_string(WORDLIST_ANIMALS),
                            Wordlist::BIP39 => name
                                .with_context(&name_context)
                                .get_wordlist_encoded_string(WORDLIST_BIP39),
                        };
                        name_context.insert(&get_forced_hash_string_for_type(
                            name_context.name_type(),
                            name,
                            format!("{}.{}", string, class),
                        ));
                    }
                }
            }
        }

        if let Some(out_names) = out_names {
            let resource_names: Vec<_> = bigfile.resource_names().collect();
            write_names(out_names, Some(resource_names.as_slice()), &name_context)?;
        }
    } else if let Some(out_names) = out_names {
        write_names(out_names, None, &name_context)?;
    }

    Ok(())
}
