//! utility functions to help handle the `:focus` internal
use {
    super::*,
    crate::{
        app::*,
        browser::BrowserState,
        command::TriggerType,
        display::Screen,
        path::{self, PathAnchor},
        pattern::InputPattern,
        preview::PreviewState,
        task_sync::Dam,
        tree::TreeOptions,
    },
    std::{
        path::{Path, PathBuf},
    },
};

pub fn on_path(
    path: PathBuf,
    screen: Screen,
    tree_options: TreeOptions,
    in_new_panel: bool,
    con: &AppContext,
) -> CmdResult {
    if in_new_panel {
        new_panel_on_path(
            path,
            screen,
            tree_options,
            PanelPurpose::None,
            con,
            HDir::Right,
        )
    } else {
        new_state_on_path(path, screen, tree_options, con)
    }
}

pub fn new_state_on_path(
    path: PathBuf,
    screen: Screen,
    tree_options: TreeOptions,
    con: &AppContext,
) -> CmdResult {
    let path = path::closest_dir(&path);
    CmdResult::from_optional_browser_state(
        BrowserState::new(path, tree_options, screen, con, &Dam::unlimited()),
        None,
        false,
    )
}

#[allow(unused_mut)]
pub fn new_panel_on_path(
    mut path: PathBuf,
    screen: Screen,
    mut tree_options: TreeOptions,
    purpose: PanelPurpose,
    con: &AppContext,
    direction: HDir,
) -> CmdResult {
    #[cfg(not(windows))]
    // We try to canonicalize the path, mostly to resolve links
    // We don't do it on Windows due to issue #809
    if let Ok(canonic) = std::fs::canonicalize(&path) {
        path = canonic;
        // If it can't be canonicalized, we'll let the panel state
        // deal with the original path
    }
    if purpose.is_preview() {
        let pattern = tree_options.pattern.tree_to_preview();
        CmdResult::NewPanel {
            state: Box::new(PreviewState::new(path, pattern, None, tree_options, con)),
            purpose,
            direction,
        }
    } else {
        let path = path::closest_dir(&path);
        // We remove the pattern on opening another browser. This will probably
        // be configuratble with a clear_pattern verb option in the future
        tree_options.pattern = InputPattern::none();
        match BrowserState::new(path, tree_options, screen, con, &Dam::unlimited()) {
            Ok(os) => CmdResult::NewPanel {
                state: Box::new(os),
                purpose,
                direction,
            },
            Err(e) => CmdResult::DisplayError(e.to_string()),
        }
    }
}

/// Compute the path to go to in case of the internal being triggered from
/// the input.
///
/// This path depends on the verb (which may hardcore the path or have a
/// pattern), from the selection,
fn path_from_input(
    verb: &Verb,
    internal_exec: &InternalExecution,
    base_path: &Path, // either the selected path or the root path
    input_arg: Option<&String>,
    app_state: &AppState,
    con: &AppContext,
) -> PathBuf {
    match (input_arg, internal_exec.arg.as_ref()) {
        (Some(input_arg), Some(verb_arg)) => {
            // The verb probably defines some patttern which uses the input.
            // For example:
            // {
            //     invocation: "gotar {path}"
            //     execution: ":focus {path}/target"
            // }
            // (or that input is useless)
            let path_builder = ExecutionStringBuilder::with_invocation(
                verb.invocation_parser.as_ref(),
                SelInfo::from_path(base_path),
                app_state,
                Some(input_arg),
            );
            path_builder.path(verb_arg, con)
        }
        (Some(input_arg), None) => {
            // the verb defines nothing
            // The :focus internal execution was triggered from the
            // input (which must be a kind of alias for :focus)
            // so we do exactly what the input asks for
            path::path_from(base_path, PathAnchor::Unspecified, input_arg)
        }
        (None, Some(verb_arg)) => {
            // the verb defines the path where to go..
            // the internal_execution specifies the path to use
            // (it may come from a configured verb whose execution is
            //  `:focus some/path`).
            // The given path may be relative hence the need for the
            // state's selection
            // (we assume a check before ensured it doesn't need an input)
            let path_builder = ExecutionStringBuilder::with_invocation(
                verb.invocation_parser.as_ref(),
                SelInfo::from_path(base_path),
                app_state,
                None,
            );
            path_builder.path(verb_arg, con)
        }
        (None, None) => {
            // user only wants to open the selected path, either in the same panel or
            // in a new one
            base_path.to_path_buf()
        }
    }
}

pub fn get_status_markdown(
    verb: &Verb,
    internal_exec: &InternalExecution,
    sel_info: SelInfo<'_>,
    invocation: &VerbInvocation,
    app_state: &AppState,
    con: &AppContext,
) -> String {
    let base_path = sel_info
        .one_path()
        .unwrap_or(&app_state.root);
    let path = path_from_input(
        verb,
        internal_exec,
        base_path,
        invocation.args.as_ref(),
        app_state,
        con,
    );
    format!("Hit *enter* to focus `{}`", path.to_string_lossy())
}

/// general implementation for verbs based on the :focus internal with optionally
/// a bang or an argument.
#[allow(clippy::too_many_arguments)]
pub fn on_internal(
    internal_exec: &InternalExecution,
    input_invocation: Option<&VerbInvocation>,
    trigger_type: TriggerType,
    selected_path: &Path,
    is_root_selected: bool,
    tree_options: TreeOptions,
    app_state: & AppState,
    cc: &CmdContext,
) -> CmdResult {
    let con = &cc.app.con;
    let screen = cc.app.screen;
    let bang = input_invocation
            .map(|inv| inv.bang)
            .unwrap_or(internal_exec.bang);
    let input_arg = input_invocation.as_ref()
        .and_then(|invocation| invocation.args.as_ref());
    match trigger_type {
        TriggerType::Input(verb) => {
            let path = path_from_input(
                verb,
                internal_exec,
                selected_path,
                input_arg,
                app_state,
                cc.app.con,
            );
            on_path(path, screen, tree_options, bang, con)
        }
        _ => {
            // the :focus internal was triggered by a key
            if let Some(arg) = &internal_exec.arg {
                // the internal_execution specifies the path to use
                // (it may come from a configured verb whose execution is
                //  `:focus some/path` or `:focus {initial-root}̀).
                // The given path may be relative hence the need for the
                // state's selection
                let path_builder = ExecutionStringBuilder::without_invocation(
                    SelInfo::from_path(selected_path),
                    app_state,
                );
                let path = path_builder.path(arg, con);
                let bang = input_invocation
                    .map(|inv| inv.bang)
                    .unwrap_or(internal_exec.bang);
                on_path(path, screen, tree_options, bang, con)
            } else if let Some(input_arg) = input_arg {
                // the :focus internal was triggered by a key, and without internal arg,
                // which means the user wants to explore the arg with purpose
                // of selecting a path
                let base_dir = selected_path.to_string_lossy();
                let path = path::path_from(&*base_dir, PathAnchor::Unspecified, input_arg);
                let arg_type = SelectionType::Any; // We might do better later
                let purpose = PanelPurpose::ArgEdition { arg_type };
                new_panel_on_path(path, screen, tree_options, purpose, con, HDir::Right)
            } else {
                // user only wants to open the selected path, either in the same panel or
                // in a new one
                let mut path = selected_path.to_path_buf();
                if !bang && is_root_selected {
                    // the selected path is the root, focusing it would do nothing, so
                    // we rather go up one level
                    if let Some(parent_path) = selected_path.parent() {
                        path = parent_path.to_path_buf();
                    }
                }
                on_path(path, screen, tree_options, bang, con)}
        }
    }
}
