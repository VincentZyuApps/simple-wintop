use std::time::Duration;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use crate::data::*;

fn format_uptime(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    if days > 0 {
        format!("{} days, {:02}:{:02}:{:02}", days, hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

pub fn render_tasks(tasks: &TasksData, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    let label = format!("Tasks: {}; {} running", tasks.total, tasks.running);
    if width < label.len() {
        spans.push(Span::styled(label, Style::default().fg(Color::White)));
        return spans;
    }

    spans.push(Span::styled(
        "Tasks: ",
        Style::default().fg(Color::Cyan),
    ));

    spans.push(Span::styled(
        format!("{}", tasks.total),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));

    spans.push(Span::styled("; ", Style::default().fg(Color::White)));

    spans.push(Span::styled(
        format!("{}", tasks.running),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));

    spans.push(Span::styled(
        " running",
        Style::default().fg(Color::Cyan),
    ));

    spans
}

pub fn render_load_avg(load: &LoadAverageData, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    let vals = format!("  {:.2}  {:.2}  {:.2}", load.one, load.five, load.fifteen);
    let line = format!("Load average:{}", vals);
    if width < line.len() {
        spans.push(Span::styled(line, Style::default().fg(Color::White)));
        return spans;
    }

    spans.push(Span::styled(
        "Load average:",
        Style::default().fg(Color::Cyan),
    ));

    spans.push(Span::styled(
        vals,
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    ));

    spans
}

pub fn render_uptime(uptime: &Duration, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    let uptime_str = format_uptime(*uptime);
    let line = format!("Uptime: {}", uptime_str);
    if width < line.len() {
        spans.push(Span::styled(line, Style::default().fg(Color::White)));
        return spans;
    }

    spans.push(Span::styled(
        "Uptime: ",
        Style::default().fg(Color::Cyan),
    ));

    spans.push(Span::styled(
        uptime_str,
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    ));

    spans
}
