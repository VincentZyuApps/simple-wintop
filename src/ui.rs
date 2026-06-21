use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::data::*;

pub fn draw(frame: &mut Frame, data: &SystemData) {
    let area = frame.area();

    if area.height < 10 || area.width < 50 {
        draw_too_small(frame, area);
        return;
    }

    let horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    draw_left_column(frame, horz[0], data);
    draw_right_column(frame, horz[1], data);
}

fn draw_left_column(frame: &mut Frame, area: Rect, data: &SystemData) {
    let n_cpus = data.cpus.len();
    let n_left = (n_cpus + 1) / 2;

    let mut rows = Vec::new();
    for _ in 0..n_left {
        rows.push(Constraint::Length(1));
    }
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(rows)
        .split(area);

    for (i, cpu) in data.cpus.iter().take(n_left).enumerate() {
        let spans = render_cpu_bar(cpu, chunks[i].width as usize);
        frame.render_widget(Paragraph::new(Line::from(spans)), chunks[i]);
    }

    let mem_spans = render_mem_bar(&data.memory, chunks[n_left].width as usize);
    frame.render_widget(Paragraph::new(Line::from(mem_spans)), chunks[n_left]);

    let swp_spans = render_swap_bar(&data.swap, chunks[n_left + 1].width as usize);
    frame.render_widget(Paragraph::new(Line::from(swp_spans)), chunks[n_left + 1]);
}

fn draw_right_column(frame: &mut Frame, area: Rect, data: &SystemData) {
    let n_cpus = data.cpus.len();
    let n_right = n_cpus / 2;
    let n_left = (n_cpus + 1) / 2;

    let mut rows = Vec::new();
    for _ in 0..n_right {
        rows.push(Constraint::Length(1));
    }
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(rows)
        .split(area);

    for (i, cpu) in data.cpus.iter().skip(n_left).enumerate() {
        let spans = render_cpu_bar(cpu, chunks[i].width as usize);
        frame.render_widget(Paragraph::new(Line::from(spans)), chunks[i]);
    }

    let off = n_right;
    let tasks_spans = render_tasks(&data.tasks, chunks[off].width as usize);
    frame.render_widget(Paragraph::new(Line::from(tasks_spans)), chunks[off]);

    let load_spans = render_load_avg(&data.load_avg, chunks[off + 1].width as usize);
    frame.render_widget(Paragraph::new(Line::from(load_spans)), chunks[off + 1]);

    let uptime_spans = render_uptime(&data.uptime, chunks[off + 2].width as usize);
    frame.render_widget(Paragraph::new(Line::from(uptime_spans)), chunks[off + 2]);
}

fn render_cpu_bar(cpu: &CpuData, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    let label_color = Color::Cyan;
    let pct_text = format!("{:>5.1}%", cpu.usage.min(100.0));
    let body_w = width
        .saturating_sub(cpu.name.len() + 3 + pct_text.len() + 1)
        .min(200);

    spans.push(Span::styled(
        format!("{}[", cpu.name),
        Style::default()
            .fg(label_color)
            .add_modifier(Modifier::BOLD),
    ));

    if body_w > 0 {
        let filled = (cpu.usage.min(100.0) / 100.0 * body_w as f64).round() as usize;
        let filled = filled.min(body_w);

        for i in 0..body_w {
            if i < filled {
                let color = if (i as f64) / body_w as f64 < 0.5 {
                    Color::Green
                } else {
                    Color::Red
                };
                spans.push(Span::styled("|", Style::default().fg(color)));
            } else {
                spans.push(Span::styled(" ", Style::default()));
            }
        }
    }

    spans.push(Span::styled(
        "]",
        Style::default()
            .fg(label_color)
            .add_modifier(Modifier::BOLD),
    ));

    let pct_color = if cpu.usage > 80.0 {
        Color::Red
    } else {
        Color::Green
    };
    spans.push(Span::styled(
        format!(" {}", pct_text),
        Style::default().fg(pct_color),
    ));

    spans
}

fn render_mem_bar(mem: &MemoryData, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let label = "Mem";

    let used_str = format_bytes(mem.used);
    let total_str = format_bytes(mem.total);
    let text = format!("{}/{}", used_str, total_str);

    let body_w = width.saturating_sub(label.len() + 3 + text.len() + 1).min(200);

    spans.push(Span::styled(
        format!("{}[", label),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    let ratio = if mem.total > 0 {
        mem.used as f64 / mem.total as f64
    } else {
        0.0
    };

    if body_w > 0 {
        let filled = (ratio * body_w as f64).round() as usize;
        let filled = filled.min(body_w);

        for i in 0..body_w {
            if i < filled {
                let color = if (i as f64) / body_w as f64 < 0.5 {
                    Color::Green
                } else if (i as f64) / body_w as f64 < 0.75 {
                    Color::Blue
                } else {
                    Color::Yellow
                };
                spans.push(Span::styled("|", Style::default().fg(color)));
            } else {
                spans.push(Span::styled(" ", Style::default()));
            }
        }
    }

    spans.push(Span::styled(
        "]",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    let used_color = if ratio > 0.9 {
        Color::Red
    } else if ratio > 0.7 {
        Color::Yellow
    } else {
        Color::Green
    };
    spans.push(Span::styled(
        format!(" {}", text),
        Style::default().fg(used_color),
    ));

    spans
}

fn render_swap_bar(swp: &SwapData, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let label = "Swp";

    let used_str = format_bytes(swp.used);
    let total_str = format_bytes(swp.total);
    let text = format!("{}/{}", used_str, total_str);

    let body_w = width.saturating_sub(label.len() + 3 + text.len() + 1).min(200);

    spans.push(Span::styled(
        format!("{}[", label),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    let ratio = if swp.total > 0 {
        swp.used as f64 / swp.total as f64
    } else {
        0.0
    };

    if body_w > 0 {
        let filled = (ratio * body_w as f64).round() as usize;
        let filled = filled.min(body_w);

        for i in 0..body_w {
            if i < filled {
                let color = if (i as f64) / body_w as f64 < 0.5 {
                    Color::Green
                } else if (i as f64) / body_w as f64 < 0.75 {
                    Color::Blue
                } else {
                    Color::Red
                };
                spans.push(Span::styled("|", Style::default().fg(color)));
            } else {
                spans.push(Span::styled(" ", Style::default()));
            }
        }
    }

    spans.push(Span::styled(
        "]",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    let used_color = if ratio > 0.9 {
        Color::Red
    } else if ratio > 0.7 {
        Color::Yellow
    } else {
        Color::Green
    };
    spans.push(Span::styled(
        format!(" {}", text),
        Style::default().fg(used_color),
    ));

    spans
}

fn render_tasks(tasks: &TasksData, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let label = "Tasks:";

    let line = format!("{} {}; {} running", label, tasks.total, tasks.running);

    if width < line.len() {
        spans.push(Span::styled(
            line,
            Style::default().fg(Color::White),
        ));
        return spans;
    }

    spans.push(Span::styled(
        format!("{} ", label),
        Style::default().fg(Color::Cyan),
    ));

    spans.push(Span::styled(
        format!("{}", tasks.total),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));

    spans.push(Span::styled(
        "; ",
        Style::default().fg(Color::White),
    ));

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

fn render_load_avg(load: &LoadAverageData, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let label = "Load average:";

    let vals = format!("  {:.2}  {:.2}  {:.2}", load.one, load.five, load.fifteen);
    let line = format!("{} {}", label, vals);

    if width < line.len() {
        spans.push(Span::styled(
            line,
            Style::default().fg(Color::White),
        ));
        return spans;
    }

    spans.push(Span::styled(
        format!("{} ", label),
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

fn render_uptime(uptime: &std::time::Duration, width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let label = "Uptime:";

    let uptime_str = format_uptime(*uptime);
    let line = format!("{} {}", label, uptime_str);

    if width < line.len() {
        spans.push(Span::styled(
            line,
            Style::default().fg(Color::White),
        ));
        return spans;
    }

    spans.push(Span::styled(
        format!("{} ", label),
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

fn draw_too_small(frame: &mut Frame, area: Rect) {
    let msg = "Terminal too small";
    let x = area.width.saturating_sub(msg.len() as u16) / 2;
    let y = area.height / 2;
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            msg,
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ))),
        Rect {
            x: area.x + x,
            y: area.y + y,
            width: msg.len() as u16,
            height: 1,
        },
    );
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
    let mut value = bytes as f64;
    let mut unit_idx = 0;
    while value >= 1024.0 && unit_idx < UNITS.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{}{}", bytes, UNITS[unit_idx])
    } else if value >= 100.0 {
        format!("{:.0}{}", value, UNITS[unit_idx])
    } else if value >= 10.0 {
        format!("{:.1}{}", value, UNITS[unit_idx])
    } else {
        format!("{:.2}{}", value, UNITS[unit_idx])
    }
}

fn format_uptime(duration: std::time::Duration) -> String {
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
