use chrono::{Duration, TimeZone, Utc};
use dawnline::action::Action;
use dawnline::models::{BlockTiming, DayState, Event, EventKind};
use dawnline::state::{ActivePane, AppState, CurrentScreen};
use dawnline::update::{Command, update};
use uuid::Uuid;

fn event_at(offset: i64, kind: EventKind) -> Event {
    Event {
        id: Uuid::now_v7(),
        at: Utc.with_ymd_and_hms(2026, 4, 30, 9, 0, 0).single().unwrap()
            + Duration::seconds(offset),
        kind,
    }
}

fn app_with_active_session_and_block() -> (AppState, Uuid, Uuid) {
    let session_id = Uuid::now_v7();
    let block_id = Uuid::now_v7();

    let day = DayState::replay(&[
        event_at(0, EventKind::SessionStarted { session_id }),
        event_at(
            1,
            EventKind::BlockAdded {
                block_id,
                title: "Stabilize CLI flow".to_string(),
                intent: Some("make it shippable".to_string()),
                timing: BlockTiming::Loose {
                    label: "now".to_string(),
                },
            },
        ),
        event_at(2, EventKind::BlockStarted { block_id }),
    ]);

    (AppState::from_day(day), session_id, block_id)
}

#[test]
fn tab_switches_panes_without_changing_mode() {
    let (mut app, _, _) = app_with_active_session_and_block();

    assert_eq!(app.current_screen, CurrentScreen::Plan);
    assert_eq!(app.active_pane, ActivePane::Timeline);

    update(&mut app, Action::NextTab);

    assert_eq!(app.current_screen, CurrentScreen::Plan);
    assert_eq!(app.active_pane, ActivePane::Tasks);

    update(&mut app, Action::NextTab);

    assert_eq!(app.current_screen, CurrentScreen::Plan);
    assert_eq!(app.active_pane, ActivePane::Timeline);
}

#[test]
fn enter_only_moves_from_plan_to_execute() {
    let (mut app, _, _) = app_with_active_session_and_block();

    update(&mut app, Action::Enter);

    assert_eq!(app.current_screen, CurrentScreen::Execute);

    update(&mut app, Action::Enter);

    assert_eq!(app.current_screen, CurrentScreen::Execute);
}

#[test]
fn finish_execution_moves_to_review_and_emits_block_and_session_finish_events() {
    let (mut app, session_id, block_id) = app_with_active_session_and_block();

    update(&mut app, Action::Enter);
    let command = update(&mut app, Action::Char('f'));

    assert_eq!(app.current_screen, CurrentScreen::Review);

    let Some(Command::AppendEvents(events)) = command else {
        panic!("expected append events command");
    };

    assert!(events.iter().any(|event| {
        matches!(
            event.kind,
            EventKind::BlockFinished { block_id: id } if id == block_id
        )
    }));

    assert!(events.iter().any(|event| {
        matches!(
            event.kind,
            EventKind::SessionFinished { session_id: id } if id == session_id
        )
    }));
}

#[test]
fn finish_day_only_emits_day_finished_from_review() {
    let (mut app, _, _) = app_with_active_session_and_block();

    assert_eq!(update(&mut app, Action::FinishDay), None);

    update(&mut app, Action::Enter);
    update(&mut app, Action::Char('f'));

    let command = update(&mut app, Action::Char('f'));

    assert!(matches!(
        command,
        Some(Command::AppendEvent(Event {
            kind: EventKind::DayFinished,
            ..
        }))
    ));
}

#[test]
fn review_command_does_not_skip_execution_finish() {
    let (mut app, _, _) = app_with_active_session_and_block();

    update(&mut app, Action::OpenCommand);
    update(&mut app, Action::Char('r'));
    update(&mut app, Action::Char('e'));
    update(&mut app, Action::Char('v'));
    update(&mut app, Action::Char('i'));
    update(&mut app, Action::Char('e'));
    update(&mut app, Action::Char('w'));

    let command = update(&mut app, Action::Enter);

    assert_eq!(command, None);
    assert_eq!(app.current_screen, CurrentScreen::Plan);
}

#[test]
fn focus_hotkey_starts_timer_and_logs_focus_from_execute() {
    let (mut app, _, _) = app_with_active_session_and_block();

    update(&mut app, Action::Enter);
    let command = update(&mut app, Action::Char('t'));

    assert!(app.focus_remaining_seconds().is_some());

    assert!(matches!(
        command,
        Some(Command::AppendEvent(Event {
            kind: EventKind::FocusLogged { minutes: 45 },
            ..
        }))
    ));
}

#[test]
fn focus_command_starts_requested_timer_from_execute() {
    let (mut app, _, _) = app_with_active_session_and_block();

    update(&mut app, Action::Enter);
    update(&mut app, Action::OpenCommand);

    for c in "focus 25".chars() {
        update(&mut app, Action::Char(c));
    }

    let command = update(&mut app, Action::Enter);

    assert_eq!(app.focus_minutes, 25);
    assert!(app.focus_remaining_seconds().is_some());

    assert!(matches!(
        command,
        Some(Command::AppendEvent(Event {
            kind: EventKind::FocusLogged { minutes: 25 },
            ..
        }))
    ));
}
