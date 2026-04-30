# Dawnline

Know what matters.

Dawnline is a focus central. It helps you shape your day, work through the current tasks and leave with a clear idea of what actually moved.

This is not a journal. This is not a TODO app.

This is an interoceptive, serious, self-directed work tool.

## Status

Dawnline is currently a concept-proof. It's being redefined each day. You can be a part of this journey. But also: expect rough edges.

## Run locally

```bash
# clone the repository
git clone https://github.com/ENIX1701/Dawnline
cd dawnline

cargo run
```

Alternatively you can get the current binary from the [releases page](https://github.com/ENIX1701/Dawnline/releases).

## What Dawnline does

Dawnline treats the day as the main object. Inside of it, you create blocks, tasks and notes.

### Plan

Plan mode lets you create:
- loose blocks
- timed blocks
- tasks (both regular and priority)
- notes

Blocks describe focus. Tasks stay actionable.

### Execute

Space to get things done.

It keeps the screen focused with only necessary information:
- what's happening now
- what comes next
- what's awaiting

### Review

Make your work count.

Review shows:
- completed and dropped tasks
- tasks to carry forward
- completed blocks
- focus time
- notes

No judgement. Just signal.

## Navigation

```text
tab     switch pane
enter   move from plan to execute
:       open command palette
a       add (opens command palette)
space   complete selected task
s       start selected block
d       drop selected task
x       remove selected task
f       finish
?       help
q       quit
```

Arrow keys move through the selected pane.

`tab` switches between the timeline and task queue.

## Command palette

Open the palette with `:` and type the commands.

Some of the available actions:
```bash
:add block now Release
:add block next Hotfix production
:add block 13:00 Deploy
:add block 15:00 18:00 Monitor service health

:add task Ship auth fix
:add priority task Patch an edge case

:note These have to be done today
:focus 45

:start Deploy
:done task 1
:drop task 2
:remove task 3

:finish
```

`finish` moves from execute into review. In review `finish` closes the day.

## CLI

Dawnline can also be used without opening the TUI.

```bash
dawnline task "Ship auth fix"
dawnline task --priority "Patch an edge case"

dawnline block "Write unit tests"
dawnline block --at 13:00 "Review last week"

dawnline note "An important note"

dawnline review
dawnline finish
```

## Storage

The state is stored in an append-only event log located at:
```bash
~/.local/share/dawnline/events.jsonl
```

The configuration is located at:
```bash
~/.config/dawnline/config.toml
```

Default config values are as follows:
```toml
tagline = "Know what matters."

[planning]
block_style = "mixed"
loose_labels = ["Now", "Next", "Later"]

[review]
scope = "day"
history_grouping = "day"

[focus]
default_minutes = 45
presets = [25, 45, 90]
mode = "countdown"

[theme]
name = "dawn"
accent = "gold"
```

## Themes

Starter themes:
- `dawn`
- `opal`
- `mist`

Supported accents:
- `gold`
- `rose`
- `sage`
- `blue`
- `lilac`

Dawnline tries to keep the terminal calm. Minimal borders. Soft colors. No noise.

## Testing

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

---

> [!IMPORTANT]
> Dawnline is intentionally small.
> 
> The goal is not to manage you whole life. It's to make the current day easier to see. To make focus happen.
