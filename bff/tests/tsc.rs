use std::collections::HashMap;

use bff::tsc::{AsoboExecutor, FileSystemScriptLoader};

use crate::path_helpers::resolve_repo_data_path;

#[derive(Default)]
struct TestData {
    seen_commands: HashMap<String, usize>,
    default_callback_count: usize,
    inline_commands: usize,
}

#[test]
fn remove_command_handler() {
    let mut executor =
        AsoboExecutor::with_user_data(FileSystemScriptLoader::new("unused"), Vec::<u32>::new());
    executor.on_command("GiveMoney1000", |executor, _command| {
        executor.user_data_mut().push(1);
        Ok(())
    });
    executor.on_command("GiveMoney1000", |executor, _command| {
        executor.user_data_mut().push(2);
        Ok(())
    });

    executor.execute_command_line("GM1000").unwrap();
    assert!(executor.remove_command("GM1000"));
    executor.execute_command_line("GiveMoney1000").unwrap();
    assert!(executor.remove_command("givemoney1000"));
    assert!(!executor.remove_command("GM1000"));
    executor.execute_command_line("GM1000").unwrap();

    assert_eq!(executor.into_user_data(), vec![2, 1]);
}

#[test]
fn remove_default_handler() {
    let mut executor = AsoboExecutor::with_user_data(FileSystemScriptLoader::new("unused"), 0usize);
    executor.on_default(|executor, _command| {
        *executor.user_data_mut() += 1;
        Ok(())
    });

    executor.execute_command_line("Unknown").unwrap();
    assert!(executor.remove_default());
    executor.execute_command_line("Unknown").unwrap();
    assert!(!executor.remove_default());

    assert_eq!(executor.into_user_data(), 1);
}

#[test]
fn execute_asobo_tsc_sample() {
    let root = resolve_repo_data_path("FUEL_tsc_sample");
    let mut executor =
        AsoboExecutor::with_user_data(FileSystemScriptLoader::new(root), TestData::default());
    executor.on_command("InlineCommand", |executor, _command| {
        executor.user_data_mut().inline_commands += 1;
        Ok(())
    });
    executor.on_default(|executor, command| {
        let data = executor.user_data_mut();
        data.default_callback_count += 1;
        *data
            .seen_commands
            .entry(command.command_name.clone())
            .or_default() += 1;
        Ok(())
    });

    executor.execute_command_line("#set _PC").unwrap();
    executor
        .execute_script_text(
            "initial_state_multiline.tsc",
            r#"
#set _MASTER
#if _PC && _MASTER
IC "quoted // text and /* text are part of this argument"
#endif
#set _BIGFILE
"#,
            Vec::<String>::new(),
        )
        .unwrap();
    executor.execute_command_line("SRC Vibrations.tsc").unwrap();
    executor.execute_command_line("BSRC user.tsc").unwrap();

    let has_pc = executor.has_variable("_PC");
    let has_master = executor.has_variable("_MASTER");
    let has_bigfile = executor.has_variable("_BIGFILE");
    let data = executor.into_user_data();
    eprintln!(
        "Asobo TSC callback activity: {} commands across {} names",
        data.default_callback_count,
        data.seen_commands.len(),
    );

    assert!(has_pc);
    assert!(has_master);
    assert!(has_bigfile);
    assert_eq!(data.inline_commands, 1);
    assert!(data.default_callback_count > 100);
    assert!(!data.seen_commands.is_empty());
}
