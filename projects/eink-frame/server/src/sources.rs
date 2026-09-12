use anyhow::Result;
use chrono::{Duration, TimeZone, Utc};
use chrono_tz::Tz;
use tracing::{info, warn};

use crate::caldav::{self, CalDav};
use crate::config::Config;
use crate::ics;
use crate::model::{Dashboard, FileShopping, FileTodo, ShoppingItem, TodoItem};

pub async fn load_dashboard(cfg: &Config) -> Result<Dashboard> {
    let tz: Tz = cfg
        .timezone
        .parse()
        .unwrap_or(chrono_tz::Europe::London);
    let today = Utc::now().with_timezone(&tz).date_naive();
    let mut dash = Dashboard::empty(&cfg.family_name, today);
    let mut notes: Vec<String> = Vec::new();

    dash.todos = read_todo_file(&cfg.resolve(&cfg.sources.todos_file))?;
    dash.shopping = read_shopping_file(&cfg.resolve(&cfg.sources.shopping_file))?;
    if !dash.todos.is_empty() {
        notes.push("local todos".into());
    }
    if !dash.shopping.is_empty() {
        notes.push("local shopping list".into());
    }

    for url in &cfg.sources.ics_urls {
        match fetch_ics(url).await {
            Ok(ics) => {
                let events = ics::parse_events(&ics, tz, today, 7)?;
                info!(url, n = events.len(), "loaded public ICS");
                merge_events(&mut dash, events);
                notes.push("public ICS".into());
            }
            Err(err) => warn!(url, %err, "public ICS failed"),
        }
    }

    if cfg.icloud_enabled() {
        match load_icloud(cfg, tz, today).await {
            Ok((events, todos, shopping, icloud_notes)) => {
                if !events.is_empty() {
                    dash.events_today.clear();
                    dash.events_week.clear();
                    merge_events(&mut dash, events);
                }
                if !todos.is_empty() {
                    dash.todos = todos;
                }
                if !shopping.is_empty() {
                    dash.shopping = shopping;
                }
                notes.extend(icloud_notes);
            }
            Err(err) => {
                warn!(%err, "iCloud CalDAV failed; keeping local/demo data");
                notes.push("iCloud unavailable — using local lists".into());
            }
        }
    } else if dash.events_today.is_empty() && dash.events_week.is_empty() {
        merge_events(&mut dash, demo_events(today));
        if dash.todos.is_empty() {
            dash.todos = demo_todos();
        }
        if dash.shopping.is_empty() {
            dash.shopping = demo_shopping();
        }
        notes.push("demo data (no iCloud credentials)".into());
    }

    dash.source_note = notes.join(" · ");
    Ok(dash)
}

async fn load_icloud(
    cfg: &Config,
    tz: Tz,
    today: chrono::NaiveDate,
) -> Result<(
    Vec<crate::model::CalendarEvent>,
    Vec<TodoItem>,
    Vec<ShoppingItem>,
    Vec<String>,
)> {
    let client = CalDav::new(&cfg.icloud)?;
    let calendars = client.list_calendars().await?;
    let start = tz
        .with_ymd_and_hms(
            chrono::Datelike::year(&today),
            chrono::Datelike::month(&today),
            chrono::Datelike::day(&today),
            0,
            0,
            0,
        )
        .single()
        .ok_or_else(|| anyhow::anyhow!("invalid timezone date"))?;
    let end = start + Duration::days(7);
    let start_utc = start.with_timezone(&Utc).format("%Y%m%dT%H%M%SZ").to_string();
    let end_utc = end.with_timezone(&Utc).format("%Y%m%dT%H%M%SZ").to_string();

    let mut events = Vec::new();
    let mut notes = Vec::new();
    for cal in caldav::match_named(&calendars, &cfg.icloud.calendars) {
        if !cal.supports_events {
            continue;
        }
        let ics = client
            .fetch_calendar_data(&cal.href, &caldav::event_report(&start_utc, &end_utc))
            .await?;
        let parsed = ics::parse_events(&ics, tz, today, 7)?;
        info!(calendar = %cal.name, n = parsed.len(), "iCloud events");
        events.extend(parsed);
        notes.push(format!("iCloud calendar “{}”", cal.name));
    }

    let todos = fetch_named_todos(&client, &calendars, &cfg.icloud.todo_list).await?;
    if !todos.is_empty() {
        notes.push(format!("iCloud list “{}”", cfg.icloud.todo_list));
    }

    let shopping_todos = fetch_named_todos(&client, &calendars, &cfg.icloud.shopping_list).await?;
    let shopping = shopping_todos
        .into_iter()
        .map(|t| ShoppingItem {
            name: t.title,
            qty: String::new(),
        })
        .collect::<Vec<_>>();
    if !shopping.is_empty() {
        notes.push(format!("iCloud shopping “{}”", cfg.icloud.shopping_list));
    }

    Ok((events, todos, shopping, notes))
}

async fn fetch_named_todos(
    client: &CalDav,
    calendars: &[caldav::CalendarRef],
    name: &str,
) -> Result<Vec<TodoItem>> {
    let mut out = Vec::new();
    for cal in caldav::match_named(calendars, &[name.to_string()]) {
        if !cal.supports_todos && cal.name.eq_ignore_ascii_case(name) {
            // Some iCloud reminder lists omit the component-set flag.
        }
        match client
            .fetch_calendar_data(&cal.href, &caldav::todo_report())
            .await
        {
            Ok(ics) => out.extend(ics::parse_todos(&ics)?),
            Err(err) => warn!(list = name, %err, "VTODO fetch failed"),
        }
    }
    Ok(out)
}

async fn fetch_ics(url: &str) -> Result<String> {
    let text = reqwest::Client::new()
        .get(url)
        .timeout(std::time::Duration::from_secs(20))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    Ok(text)
}

fn merge_events(dash: &mut Dashboard, events: Vec<crate::model::CalendarEvent>) {
    for ev in events {
        if ev.day_label == "Today" {
            dash.events_today.push(ev);
        } else {
            dash.events_week.push(ev);
        }
    }
}

fn read_todo_file(path: &std::path::Path) -> Result<Vec<TodoItem>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path)?;
    let items: Vec<FileTodo> = serde_json::from_str(&text)?;
    Ok(items
        .into_iter()
        .map(|t| TodoItem {
            title: t.title,
            done: t.done,
        })
        .collect())
}

fn read_shopping_file(path: &std::path::Path) -> Result<Vec<ShoppingItem>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path)?;
    let items: Vec<FileShopping> = serde_json::from_str(&text)?;
    Ok(items
        .into_iter()
        .map(|t| ShoppingItem {
            name: t.name,
            qty: t.qty,
        })
        .collect())
}

fn demo_events(today: chrono::NaiveDate) -> Vec<crate::model::CalendarEvent> {
    use crate::model::CalendarEvent;
    vec![
        CalendarEvent {
            start: "08:15".into(),
            title: "School run".into(),
            who: "Alex".into(),
            all_day: false,
            day_label: "Today".into(),
        },
        CalendarEvent {
            start: "18:30".into(),
            title: "Dinner at Sam’s".into(),
            who: "".into(),
            all_day: false,
            day_label: "Today".into(),
        },
        CalendarEvent {
            start: String::new(),
            title: "Swim".into(),
            who: "Alex".into(),
            all_day: true,
            day_label: "Tomorrow".into(),
        },
        CalendarEvent {
            start: "16:00".into(),
            title: "Parents’ evening".into(),
            who: "".into(),
            all_day: false,
            day_label: (today + Duration::days(3)).format("%a %-d").to_string(),
        },
    ]
}

fn demo_todos() -> Vec<TodoItem> {
    vec![
        TodoItem {
            title: "Email the school office".into(),
            done: false,
        },
        TodoItem {
            title: "Book MOT".into(),
            done: false,
        },
        TodoItem {
            title: "Return library books".into(),
            done: false,
        },
    ]
}

fn demo_shopping() -> Vec<ShoppingItem> {
    vec![
        ShoppingItem {
            name: "Milk".into(),
            qty: "2".into(),
        },
        ShoppingItem {
            name: "Sourdough".into(),
            qty: "".into(),
        },
        ShoppingItem {
            name: "Apples".into(),
            qty: "6".into(),
        },
        ShoppingItem {
            name: "Dishwasher tablets".into(),
            qty: "".into(),
        },
    ]
}
