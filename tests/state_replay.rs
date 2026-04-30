use chrono::{Duration, TimeZone, Utc};
use dawnline::models::{BlockStatus, BlockTiming, DayState, Event, EventKind, TaskStatus};
use dawnline::state::AppState;
use dawnline::store::EventStore;
use uuid::Uuid;

fn event_at(offset: i64, kind: EventKind) -> Event {
    Event {
        id: Uuid::now_v7(),
        at: Utc.with_ymd_and_hms(2026, 4, 30, 9, 0, 0).single().unwrap()
            + Duration::seconds(offset),
        kind,
    }
}

fn temp_store() -> EventStore {
    EventStore {
        path: std::env::temp_dir().join(format!("dawnline-test-{}.jsonl", Uuid::now_v7())),
    }
}

#[test]
fn completed_task_is_hidden_from_active_tasks_but_kept_in_history() {
    let task_id = Uuid::now_v7();

    let events = vec![
        event_at(
            0,
            EventKind::TaskAdded {
                task_id,
                title: "Finish event replay".to_string(),
                priority: false,
            },
        ),
        event_at(1, EventKind::TaskCompleted { task_id }),
    ];

    let state = DayState::replay(&events);

    assert!(state.active_tasks().is_empty());
    assert_eq!(state.completed_tasks().len(), 1);
    assert_eq!(state.tasks[0].status, TaskStatus::Done);
    assert!(state.tasks[0].closed_at.is_some());
    assert_eq!(state.history.len(), 2);
}

#[test]
fn dropped_and_removed_tasks_leave_active_views_but_remain_in_history() {
    let dropped_id = Uuid::now_v7();
    let removed_id = Uuid::now_v7();

    let events = vec![
        event_at(
            0,
            EventKind::TaskAdded {
                task_id: dropped_id,
                title: "Drop this".to_string(),
                priority: false,
            },
        ),
        event_at(
            1,
            EventKind::TaskAdded {
                task_id: removed_id,
                title: "Remove this".to_string(),
                priority: false,
            },
        ),
        event_at(
            2,
            EventKind::TaskDropped {
                task_id: dropped_id,
            },
        ),
        event_at(
            3,
            EventKind::TaskRemoved {
                task_id: removed_id,
            },
        ),
    ];

    let state = DayState::replay(&events);

    assert!(state.active_tasks().is_empty());
    assert_eq!(state.dropped_tasks().len(), 1);
    assert_eq!(state.tasks[0].status, TaskStatus::Dropped);
    assert_eq!(state.tasks[1].status, TaskStatus::Removed);
    assert_eq!(state.history.len(), 4);
}

#[test]
fn priority_tasks_sort_above_normal_tasks_without_losing_created_order() {
    let normal_early = Uuid::now_v7();
    let priority = Uuid::now_v7();
    let normal_late = Uuid::now_v7();

    let events = vec![
        event_at(
            0,
            EventKind::TaskAdded {
                task_id: normal_early,
                title: "Normal early".to_string(),
                priority: false,
            },
        ),
        event_at(
            1,
            EventKind::TaskAdded {
                task_id: priority,
                title: "Priority".to_string(),
                priority: true,
            },
        ),
        event_at(
            2,
            EventKind::TaskAdded {
                task_id: normal_late,
                title: "Normal late".to_string(),
                priority: false,
            },
        ),
    ];

    let state = DayState::replay(&events);
    let tasks = state.active_tasks();

    assert_eq!(tasks[0].id, priority);
    assert_eq!(tasks[1].id, normal_early);
    assert_eq!(tasks[2].id, normal_late);
}

#[test]
fn block_start_and_finish_update_current_block() {
    let first = Uuid::now_v7();
    let second = Uuid::now_v7();

    let events = vec![
        event_at(
            0,
            EventKind::BlockAdded {
                block_id: first,
                title: "Write parser tests".to_string(),
                intent: None,
                timing: BlockTiming::Loose {
                    label: "now".to_string(),
                },
            },
        ),
        event_at(
            1,
            EventKind::BlockAdded {
                block_id: second,
                title: "Review analytics".to_string(),
                intent: None,
                timing: BlockTiming::Exact {
                    start: "13:00".to_string(),
                    end: None,
                },
            },
        ),
        event_at(2, EventKind::BlockStarted { block_id: first }),
        event_at(3, EventKind::BlockFinished { block_id: first }),
        event_at(4, EventKind::BlockStarted { block_id: second }),
    ];

    let state = DayState::replay(&events);

    assert_eq!(state.active_block().unwrap().id, second);
    assert_eq!(state.completed_blocks().len(), 1);
    assert_eq!(state.blocks[0].status, BlockStatus::Done);
    assert_eq!(state.blocks[1].status, BlockStatus::Active);
}

#[test]
fn day_finished_closes_active_session() {
    let session_id = Uuid::now_v7();

    let events = vec![
        event_at(0, EventKind::SessionStarted { session_id }),
        event_at(1, EventKind::DayFinished),
    ];

    let state = DayState::replay(&events);

    assert!(state.day_finished);
    assert_eq!(state.current_session_id, None);
    assert!(state.sessions[0].finished_at.is_some());
}

#[test]
fn store_does_not_start_new_session_after_day_is_finished() -> color_eyre::Result<()> {
    let mut store = temp_store();
    let session_id = Uuid::now_v7();

    store.append(Event::new(EventKind::SessionStarted { session_id }))?;
    store.append(Event::new(EventKind::DayFinished))?;
    store.ensure_session()?;

    let state = store.load_state()?;
    let started_sessions = state
        .history
        .iter()
        .filter(|event| matches!(event.kind, EventKind::SessionStarted { .. }))
        .count();

    assert!(state.day_finished);
    assert_eq!(started_sessions, 1);

    let _ = std::fs::remove_file(&store.path);
    Ok(())
}

#[test]
fn store_replays_appended_events() -> color_eyre::Result<()> {
    let mut store = temp_store();
    let task_id = Uuid::now_v7();

    store.append(Event::new(EventKind::TaskAdded {
        task_id,
        title: "Ship auth fix".to_string(),
        priority: true,
    }))?;

    let state = store.load_state()?;

    assert_eq!(state.active_tasks().len(), 1);
    assert_eq!(state.active_tasks()[0].id, task_id);

    let _ = std::fs::remove_file(&store.path);
    Ok(())
}

#[test]
fn palette_start_starts_block_even_when_no_block_is_active() -> color_eyre::Result<()> {
    let mut store = temp_store();
    let block_id = Uuid::now_v7();

    store.append(Event::new(EventKind::BlockAdded {
        block_id,
        title: "Polish review screen".to_string(),
        intent: None,
        timing: BlockTiming::Loose {
            label: "next".to_string(),
        },
    }))?;

    let message = store.run_palette_command("start Polish")?;
    let state = store.load_state()?;

    assert_eq!(message, "Block started");
    assert_eq!(state.active_block().unwrap().id, block_id);

    let _ = std::fs::remove_file(&store.path);
    Ok(())
}

#[test]
fn palette_accepts_leading_colon_and_exact_block_times() -> color_eyre::Result<()> {
    let mut store = temp_store();

    let message = store.run_palette_command(":add block 13:00 14:30 Review metrics")?;
    let state = store.load_state()?;

    assert_eq!(message, "Block added");
    assert_eq!(state.blocks.len(), 1);

    match &state.blocks[0].timing {
        BlockTiming::Exact { start, end } => {
            assert_eq!(start, "13:00");
            assert_eq!(end.as_deref(), Some("14:30"));
        }
        _ => panic!("expected exact block timing"),
    }

    let _ = std::fs::remove_file(&store.path);
    Ok(())
}

#[test]
fn visible_blocks_group_loose_and_exact_blocks_for_timeline_navigation() {
    let later = Uuid::now_v7();
    let next = Uuid::now_v7();
    let afternoon = Uuid::now_v7();
    let morning = Uuid::now_v7();
    let now = Uuid::now_v7();

    let state = DayState::replay(&[
        event_at(
            0,
            EventKind::BlockAdded {
                block_id: later,
                title: "Package release notes".to_string(),
                intent: None,
                timing: BlockTiming::Loose {
                    label: "later".to_string(),
                },
            },
        ),
        event_at(
            1,
            EventKind::BlockAdded {
                block_id: next,
                title: "Polish review screen".to_string(),
                intent: None,
                timing: BlockTiming::Loose {
                    label: "next".to_string(),
                },
            },
        ),
        event_at(
            2,
            EventKind::BlockAdded {
                block_id: afternoon,
                title: "Review metrics".to_string(),
                intent: None,
                timing: BlockTiming::Exact {
                    start: "14:30".to_string(),
                    end: None,
                },
            },
        ),
        event_at(
            3,
            EventKind::BlockAdded {
                block_id: morning,
                title: "Write parser tests".to_string(),
                intent: None,
                timing: BlockTiming::Exact {
                    start: "09:30".to_string(),
                    end: None,
                },
            },
        ),
        event_at(
            4,
            EventKind::BlockAdded {
                block_id: now,
                title: "Stabilize CLI flow".to_string(),
                intent: None,
                timing: BlockTiming::Loose {
                    label: "now".to_string(),
                },
            },
        ),
    ]);

    let app = AppState::from_day(state);
    let ids: Vec<Uuid> = app
        .visible_blocks()
        .into_iter()
        .map(|block| block.id)
        .collect();

    assert_eq!(ids, vec![now, next, morning, afternoon, later]);
}
