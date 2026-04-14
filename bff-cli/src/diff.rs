use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::BigFile;
use bff::bigfile::resource::Resource;
use bff::names::NameContext;

use crate::error::BffCliResult;
use crate::extract::{probe_bigfile_name_context, read_bigfile, read_bigfile_names};

struct ResolvedResource<'a> {
    link_name: Option<String>,
    resource: &'a Resource,
}

fn read_name_file(name_path: &Path, name_context: &NameContext) -> BffCliResult<()> {
    let f = File::open(name_path)?;
    let mut reader = BufReader::new(f);
    name_context.read(&mut reader)?;
    Ok(())
}

fn load_bigfile(
    bigfile_path: &Path,
    name_path: &Option<PathBuf>,
) -> BffCliResult<(BigFile, NameContext)> {
    let name_context = probe_bigfile_name_context(bigfile_path, &None, &None)?;
    read_bigfile_names(bigfile_path, &name_context)?;
    if let Some(name_path) = name_path {
        read_name_file(name_path, &name_context)?;
    }
    let bigfile = read_bigfile(bigfile_path, &None, &None, &name_context)?;
    Ok((bigfile, name_context))
}

fn resolve_resources<'a>(
    bigfile: &'a BigFile,
    name_context: &NameContext,
    side: &'static str,
) -> BffCliResult<BTreeMap<String, ResolvedResource<'a>>> {
    let mut resources = BTreeMap::new();

    for resource in bigfile.resources.values() {
        let resource_name = resource.name.with_context(name_context).to_string();
        let class_name = resource.class_name.with_context(name_context).to_string();
        let full_name = format!("{resource_name}.{class_name}");
        let resolved_resource = ResolvedResource {
            link_name: resource
                .link_name
                .as_ref()
                .map(|name| name.with_context(name_context).to_string()),
            resource,
        };

        assert!(
            resources
                .insert(full_name.clone(), resolved_resource)
                .is_none(),
            "duplicate resolved resource name {full_name} in {side}"
        );
    }

    Ok(resources)
}

fn display_link_name(link_name: &Option<String>) -> &str {
    link_name.as_deref().unwrap_or("<none>")
}

fn describe_data_change(old_resource: &Resource, new_resource: &Resource) -> String {
    let old_size = old_resource.size();
    let new_size = new_resource.size();

    if old_size == new_size {
        format!("data changed ({old_size} bytes)")
    } else {
        format!("data: {old_size} -> {new_size} bytes")
    }
}

fn describe_changes(
    old_resource: &ResolvedResource<'_>,
    new_resource: &ResolvedResource<'_>,
) -> Vec<String> {
    let mut changes = Vec::new();

    if old_resource.link_name != new_resource.link_name {
        changes.push(format!(
            "link: {} -> {}",
            display_link_name(&old_resource.link_name),
            display_link_name(&new_resource.link_name)
        ));
    }

    if old_resource.resource.data != new_resource.resource.data {
        changes.push(describe_data_change(
            old_resource.resource,
            new_resource.resource,
        ));
    }

    changes
}

pub fn diff(
    old_bigfile_path: &Path,
    new_bigfile_path: &Path,
    old_name_path: &Option<PathBuf>,
    new_name_path: &Option<PathBuf>,
) -> BffCliResult<()> {
    let (old_bigfile, old_name_context) = load_bigfile(old_bigfile_path, old_name_path)?;
    let (new_bigfile, new_name_context) = load_bigfile(new_bigfile_path, new_name_path)?;

    let old_resources = resolve_resources(&old_bigfile, &old_name_context, "old BigFile")?;
    let new_resources = resolve_resources(&new_bigfile, &new_name_context, "new BigFile")?;

    let added = new_resources
        .keys()
        .filter(|name| !old_resources.contains_key(*name))
        .cloned()
        .collect::<Vec<_>>();
    let removed = old_resources
        .keys()
        .filter(|name| !new_resources.contains_key(*name))
        .cloned()
        .collect::<Vec<_>>();
    let changed = old_resources
        .iter()
        .filter_map(|(name, old_resource)| {
            let new_resource = new_resources.get(name)?;
            let changes = describe_changes(old_resource, new_resource);
            (!changes.is_empty()).then(|| format!("{name} ({})", changes.join(", ")))
        })
        .collect::<Vec<_>>();

    let mut stdout = std::io::stdout().lock();

    if added.is_empty() && removed.is_empty() && changed.is_empty() {
        writeln!(stdout, "No differences found.")?;
        return Ok(());
    }

    let mut sections = [("Added", added), ("Removed", removed), ("Changed", changed)]
        .into_iter()
        .filter(|(_, lines)| !lines.is_empty())
        .peekable();

    while let Some((title, lines)) = sections.next() {
        writeln!(stdout, "{title}:")?;
        for line in lines {
            writeln!(stdout, "  {line}")?;
        }
        if sections.peek().is_some() {
            writeln!(stdout)?;
        }
    }

    Ok(())
}
