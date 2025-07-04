//! declare the internal functions which may be used in verbs.
//! They don't take any user argument other than the selection
//! (this may change if the needs arise).
//! They can be called as ":some_name" from builtin verbs and
//! from configured verbs.

use {
    crate::errors::ConfError,
};

macro_rules! Internals {
    (
        $($name:ident: $description:literal $need_path:literal,)*
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum Internal {
            $($name,)*
        }
        impl Internal {
            pub fn try_from(verb: &str) -> Result<Internal, ConfError> {
                use Internal::*;
                match verb {
                    $(stringify!($name) => Ok($name),)*
                    _ => Err(ConfError::UnknownInternal{ verb: verb.to_string() }),
                }
            }
        }
        impl Internal {
            pub fn name(self) -> &'static str {
                use Internal::*;
                match self {
                    $($name => stringify!($name),)*
                }
            }
            pub fn description(self) -> &'static str {
                use Internal::*;
                match self {
                    $($name => $description,)*
                }
            }
            pub fn need_path(self) -> bool {
                use Internal::*;
                match self {
                    $($name => $need_path,)*
                }
            }
        }
    }
}


// internals:
//  name: "description" needs_a_path
Internals! {
    apply_flags: "apply flags (eg `-sd` to show sizes and dates)" false,
    back: "revert to the previous state (mapped to *esc*)" false,
    default_layout: "restore default panel sizes" false,
    clear_output: "clear the --verb-output file" false,
    clear_stage: "empty the staging area" false,
    close_panel_cancel: "close the panel, not using the selected path" false,
    close_panel_ok: "close the panel, validating the selected path" false,
    close_preview: "close the preview panel" false,
    close_staging_area: "close the staging area panel" false,
    copy_line: "copy selected line (in tree or preview)" true,
    copy_path: "copy path to system clipboard" true,
    escape: "escape from edition, completion, page, etc." false,
    filesystems: "list mounted filesystems" false,
    focus: "display the directory (mapped to *enter*)" true,
    focus_staging_area_no_open: "focus the staging area if already open" false,
    help: "display broot's help" false,
    input_clear: "empty the input" false,
    input_del_char_below: "delete the char left at the cursor's position" false,
    input_del_char_left: "delete the char left of the cursor" false,
    input_del_word_left: "delete the word left of the cursor" false,
    input_del_word_right: "delete the word right of the cursor" false,
    input_go_left: "move the cursor to the left" false,
    input_go_right: "move the cursor to the right" false,
    input_go_to_end: "move the cursor to the end of input" false,
    input_go_to_start: "move the cursor to the start of input" false,
    input_go_word_left: "move the cursor one word to the left" false,
    input_go_word_right: "move the cursor one word to the right" false,
    input_paste: "paste the clipboard content into the input" false,
    input_selection_copy: "copy the selected part of the input into the selection" false,
    input_selection_cut: "cut the selected part of the input into the selection" false,
    line_down: "move one line down" false,
    line_down_no_cycle: "move one line down" false,
    line_up: "move one line up" false,
    line_up_no_cycle: "move one line up" false,
    mode_command: "enter the command mode" false,
    mode_input: "enter the input mode" false,
    move_panel_divider: "move a panel divider" false,
    next_dir: "select the next directory" false,
    next_match: "select the next match" false,
    next_same_depth: "select the next file at the same depth" false,
    no_sort: "don't sort" false,
    open_leave: "open file or directory according to OS (quit broot)" true,
    open_preview: "open the preview panel" true,
    open_staging_area: "open the staging area" false,
    open_stay: "open file or directory according to OS (stay in broot)" true,
    open_stay_filter: "display the directory, keeping the current pattern" true,
    open_trash: "show the content of the trash" false,
    page_down: "scroll one page down" false,
    page_up: "scroll one page up" false,
    panel_left: "focus or open panel on left" false,
    panel_left_no_open: "focus panel on left" false,
    panel_right: "focus or open panel on right" false,
    panel_right_no_open: "focus panel on right" false,
    parent: "move to the parent directory" false,
    preview_binary: "preview the selection as binary" true,
    preview_image: "preview the selection as image" true,
    preview_text: "preview the selection as text" true,
    preview_tty: "preview the selection as tty" true,
    previous_dir: "select the previous directory" false,
    previous_match: "select the previous match" false,
    previous_same_depth: "select the previous file at the same depth" false,
    print_path: "print path and leaves broot" true,
    print_relative_path: "print relative path and leaves broot" true,
    print_tree: "print tree and leaves broot" true,
    quit: "quit Broot" false,
    refresh: "refresh tree and clear size cache" false,
    delete_trashed_file: "irreversibly delete a file which is in the trash" false,
    restore_trashed_file: "restore a file which is in the trash" false,
    purge_trash: "irreversibly delete the trash's content" false,
    root_down: "move tree root down" true,
    root_up: "move tree root up" true,
    select: "select a file by path" true,
    show: "reveal and select a file by path" true,
    select_first: "select the first item" false,
    select_last: "select the last item" false,
    set_panel_width: "set the width of a panel" false,
    set_syntax_theme: "set the theme of code preview" false,
    sort_by_count: "sort by count" false,
    sort_by_date: "sort by date" false,
    sort_by_size: "sort by size" false,
    sort_by_type: "sort by type" false,
    sort_by_type_dirs_first: "sort by type, dirs first" false,
    sort_by_type_dirs_last: "sort by type, dirs last" false,
    stage: "add selection to staging area" true,
    stage_all_directories: "stage all matching directories" true,
    stage_all_files: "stage all matching files" true,
    start_end_panel: "either open or close an additional panel" true,
    toggle_counts: "toggle showing number of files in directories" false,
    toggle_dates: "toggle showing last modified dates" false,
    toggle_device_id: "toggle showing device id" false,
    toggle_files: "toggle showing files (or just folders)" false,
    toggle_git_file_info: "toggle display of git file information" false,
    toggle_git_ignore: "toggle use of .gitignore and .ignore" false,
    toggle_git_status: "toggle showing only files relevant for git status" false,
    toggle_hidden: "toggle showing hidden files" false,
    toggle_ignore: "toggle use of .gitignore and .ignore" false,
    toggle_perm: "toggle showing file permissions" false,
    toggle_preview: "open/close the preview panel" false,
    toggle_root_fs: "toggle showing filesystem info on top" false,
    set_max_depth: "set the maximum directory depth shown" false,
    unset_max_depth: "clear the max_depth" false,
    toggle_second_tree: "toggle display of a second tree panel" true,
    toggle_sizes: "toggle showing sizes" false,
    toggle_stage: "add or remove selection to staging area" true,
    toggle_staging_area: "open/close the staging area panel" false,
    toggle_tree: "toggle showing more than one level of the tree" true,
    toggle_trim_root: "toggle removing nodes at first level too" false,
    total_search: "search again but on all children" false,
    search_again: "either put back last search, or search deeper" false,
    trash: "move file to system trash" true,
    unstage: "remove selection from staging area" true,
    up_tree: "focus the parent of the current root" true,
    write_output: "write the argument to the --verb-output file" false,
    //restore_pattern: "restore a pattern which was just removed" false,
}

impl Internal {
    pub fn invocation_pattern(self) -> &'static str {
        match self {
            Self::apply_flags => r"-(?P<flags>\w+)?",
            Self::focus => r"focus (?P<path>.*)?",
            Self::select => r"select (?P<path>.*)?",
            Self::show => r"show (?P<path>.*)?",
            Self::line_down => r"line_down (?P<count>\d*)?",
            Self::line_up => r"line_up (?P<count>\d*)?",
            Self::line_down_no_cycle => r"line_down_no_cycle (?P<count>\d*)?",
            Self::line_up_no_cycle => r"line_up_no_cycle (?P<count>\d*)?",
            Self::move_panel_divider => r"move_panel_divider (?P<idx>\d+) (?P-?<dx>\d+)",
            Self::set_panel_width => r"set_panel_width (?P<idx>\d+) (?P<width>\d+)",
            Self::set_max_depth => r"set_max_depth (?P<depth>\d+)",
            Self::set_syntax_theme => r"set_syntax_theme {theme:theme}",
            Self::write_output => r"write_output (?P<line>.*)",
            _ => self.name(),
        }
    }
    pub fn exec_pattern(self) -> &'static str {
        match self {
            Self::apply_flags => r"apply_flags {flags}",
            Self::focus => r"focus {path}",
            Self::line_down => r"line_down {count}",
            Self::line_up => r"line_up {count}",
            Self::line_down_no_cycle => r"line_down_no_cycle {count}",
            Self::line_up_no_cycle => r"line_up_no_cycle {count}",
            Self::move_panel_divider => r"move_panel_divider {idx} {dx}",
            Self::set_panel_width => r"set_panel_width {idx} {width}",
            Self::write_output => r"write_output {line}",
            _ => self.name(),
        }
    }
    pub fn needs_selection(self, arg: &Option<String>) -> bool {
        match self {
            Internal::focus => arg.is_none(),
            _ => self.need_path(),
        }
    }
    pub fn is_input_related(self) -> bool {
        match self {
            Self::input_clear => true,
            Self::input_del_char_below => true,
            Self::input_del_char_left => true,
            Self::input_del_word_left => true,
            Self::input_del_word_right => true,
            Self::input_go_left => true,
            Self::input_go_right => true,
            Self::input_go_to_end => true,
            Self::input_go_to_start => true,
            Self::input_go_word_left => true,
            Self::input_go_word_right => true,
            Self::input_paste => true,
            Self::input_selection_copy => true,
            Self::input_selection_cut => true,
            _ => false,
        }
    }
}
