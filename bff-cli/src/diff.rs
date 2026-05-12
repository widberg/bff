use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use bff::BufReader;
use bff::bigfile::BigFile;
use bff::bigfile::resource::{Resource, ResourceData};
use bff::names::NameContext;

use crate::error::BffCliResult;
use crate::shared::{probe_bigfile_name_context, read_bigfile, read_bigfile_names};

struct ResolvedResource<'a> {
    link_name: Option<String>,
    resource: &'a Resource,
}

fn read_name_file(name_path: &Path, name_context: &mut NameContext) -> BffCliResult<()> {
    let f = File::open(name_path)?;
    let mut reader = BufReader::new(f);
    name_context.read(&mut reader)?;
    Ok(())
}

fn load_bigfile(
    bigfile_path: &Path,
    name_path: Option<&Path>,
) -> BffCliResult<(BigFile, NameContext)> {
    let mut name_context = probe_bigfile_name_context(bigfile_path, None, None)?;
    read_bigfile_names(bigfile_path, &mut name_context)?;
    if let Some(name_path) = name_path {
        read_name_file(name_path, &mut name_context)?;
    }
    let bigfile = read_bigfile(bigfile_path, None, None, &name_context)?;
    Ok((bigfile, name_context))
}

fn resolve_resources<'a>(
    bigfile: &'a BigFile,
    name_context: &NameContext,
    side: &'static str,
) -> BffCliResult<BTreeMap<String, ResolvedResource<'a>>> {
    let mut resources = BTreeMap::new();

    for bff_resource in bigfile.bff_resources() {
        let resource_name = bff_resource
            .resource
            .name
            .with_context(name_context)
            .to_string();
        let class_name = bff_resource
            .resource
            .class_name
            .with_context(name_context)
            .to_string();
        let full_name = format!("{resource_name}.{class_name}");
        let resolved_resource = ResolvedResource {
            link_name: bff_resource
                .resource
                .link_name
                .as_ref()
                .map(|name| name.with_context(name_context).to_string()),
            resource: bff_resource.resource,
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

fn display_link_name(link_name: Option<&str>) -> &str {
    link_name.unwrap_or("<none>")
}

fn describe_size_change(label: &str, old_size: usize, new_size: usize) -> String {
    if old_size == new_size {
        format!("{label} changed ({old_size} bytes)")
    } else {
        format!("{label}: {old_size} -> {new_size} bytes")
    }
}

fn describe_split_part_size_change(label: &str, old_size: usize, new_size: usize) -> String {
    if old_size == new_size {
        format!("{label}: {old_size} bytes")
    } else {
        format!("{label}: {old_size} -> {new_size} bytes")
    }
}

fn describe_data_change(old_resource: &Resource, new_resource: &Resource) -> String {
    match (&old_resource.data, &new_resource.data) {
        (ResourceData::Data(old_data), ResourceData::Data(new_data)) => {
            describe_size_change("data", old_data.len(), new_data.len())
        }
        (
            ResourceData::SplitData {
                link_header: old_link_header,
                body: old_body,
            },
            ResourceData::SplitData {
                link_header: new_link_header,
                body: new_body,
            },
        ) => format!(
            "split data ({}, {})",
            describe_split_part_size_change(
                "link_header",
                old_link_header.len(),
                new_link_header.len()
            ),
            describe_split_part_size_change("body", old_body.len(), new_body.len())
        ),
        (ResourceData::Data(old_data), ResourceData::SplitData { link_header, body }) => format!(
            "data format: Data ({} bytes) -> SplitData (link_header: {} bytes, body: {} bytes)",
            old_data.len(),
            link_header.len(),
            body.len()
        ),
        (ResourceData::SplitData { link_header, body }, ResourceData::Data(new_data)) => format!(
            "data format: SplitData (link_header: {} bytes, body: {} bytes) -> Data ({} bytes)",
            link_header.len(),
            body.len(),
            new_data.len()
        ),
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
            display_link_name(old_resource.link_name.as_deref()),
            display_link_name(new_resource.link_name.as_deref())
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
    old_name_path: Option<&Path>,
    new_name_path: Option<&Path>,
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
