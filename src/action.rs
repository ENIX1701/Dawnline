#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    // system lifecycle
    Tick,
    Quit,
    Resize(u16, u16),

    // global navigation
    NextTab,
    PrevTab,
    ToggleHelp,

    // raw input
    Enter,
    Esc,
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Char(char),

    // dawnline actions
    OpenCommand,
    SubmitCommand,
    StartSelectedBlock,
    CompleteSelectedTask,
    DropSelectedTask,
    RemoveSelectedTask,
    StartNewSession,
    FinishSession,
    FinishDay,

    // storage
    ReceiveStoreResult(Result<String, String>),
}
