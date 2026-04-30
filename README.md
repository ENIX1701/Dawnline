
A minimal, clean terminal focus center. In your CLI.

---

The concept is simple. An app that supports your focus in times of constant distraction.

### Initial product spec

#### Modes

##### Plan

Used before or during work to shape the day

Allows users to create time blocks, loose blocks, tasks (priority and normal) and notes. Blocks describe intended focus. Tasks are actionable

##### Execute

Main work mode. Shows what matters now and what's next.

##### Review

Used when finishing work. Shows you what got done, what's still pending and what's important. 

#### Key concepts

##### Day

Calendar-level view of sessions and other events.

##### Session

A focused work period. Each day can have multiple sessions.

##### Block

Planned scope of attention. One or more blocks make up a session

##### Task

An actionable item. These are to be completed within blocks

Can be priority or normal

##### Note

Can be added during planning, execution or review.

#### Command palette

Each command opens with `:`

example commands (obviosly more to come; this is just a teaser):
```bash
:add block 13:00 14:30 Plan the holiday
:add block next Fix onboarding
:focus 45
:start Plan the holiday
:finish
:drop task 3
:review
```

#### Keymap

```
tab     switch pane
a       add item
space   complete selected task
s       start block
d       delete block
:       command input
r       review
f       focus
?       help
q       quit
```

---

Test scenario:

##### 1. Open app, plan mode
##### 2. Open the command palette (`:`) and add stuff

```bash
:add block 14:00 22:00 Dawnline release
:add block next Create README
:add priority task Finish manual testing
:add task Prepare deploy
:note Almost there
```

##### 3. `tab` to switch panes
##### 4. Arrow keys to move around. Select a block and press `s` to select
##### 5. `enter` to move into Execute
##### 6. Complete one task with `space`
##### 7. In command palette:
```bash
:focus 5
:note Almost done for the day
```
##### 8. `f` to move into review
##### 9. `f` again to finish the day

---

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```
