use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::Path;

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::{BffResource, Resource};
use bff::class::Class;
use bff::names::NameContext;
use bff::traits::TryYourBest;

use crate::error::BffCliResult;

pub fn try_your_best(path: &Path) -> BffCliResult<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    if let Ok(name_type) = bff::bigfile::resource::BffResourceHeader::probe_name_type(&mut reader) {
        reader.rewind()?;
        let name_context = NameContext::new(name_type);
        if let Ok(bff_resource) = BffResource::read(&mut reader, &name_context) {
            let report = <Class as TryYourBest<&Resource>>::report(
                &bff_resource.resource,
                bff_resource.header.platform,
            );
            println!("{}", report);
            return Ok(());
        }
        reader.rewind()?;
    }

    {
        let platform = path
            .extension()
            .and_then(|e| e.try_into().ok())
            .unwrap_or(Platform::PC);
        let report = BigFile::report(&mut reader, platform);
        println!("{}", report);
    }

    Ok(())
}
