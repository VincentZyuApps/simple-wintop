use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use crate::data::*;

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

fn bar_spans(
    label: &str,
    body_w: usize,
    ratio: f64,
    pct_color: Color,
    suffix: String,
) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    let label_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    spans.push(Span::styled(format!("{}[", label), label_style));

    if body_w > 0 {
        let filled = (ratio * body_w as f64).round() as usize;
        let filled = filled.min(body_w);

        for i in 0..body_w {
            if i < filled {
                let pos = i as f64 / (body_w as f64);
                let color = if pos < 0.5 {
                    Color::Green
                } else if pos < 0.75 {
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

    spans.push(Span::styled("]", label_style));
    spans.push(Span::styled(
        format!(" {}", suffix),
        Style::default().fg(pct_color),
    ));

    spans
}

pub fn render_cpu_bar(cpu: &CpuData, width: usize) -> Vec<Span<'static>> {
    let pct_text = format!("{:>5.1}%", cpu.usage.min(100.0));
    let body_w = width
        .saturating_sub(cpu.name.len() + 3 + pct_text.len() + 1)
        .min(200);

    let mut spans: Vec<Span<'static>> = Vec::new();

    let label_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    spans.push(Span::styled(format!("{}[", cpu.name), label_style));

    if body_w > 0 {
        let filled = (cpu.usage.min(100.0) / 100.0 * body_w as f64).round() as usize;
        let filled = filled.min(body_w);

        for i in 0..body_w {
            if i < filled {
                let pos = i as f64 / (body_w as f64);
                let color = if pos < 0.5 { Color::Green } else { Color::Red };
                spans.push(Span::styled("|", Style::default().fg(color)));
            } else {
                spans.push(Span::styled(" ", Style::default()));
            }
        }
    }

    spans.push(Span::styled("]", label_style));

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

pub fn render_mem_bar(mem: &MemoryData, width: usize) -> Vec<Span<'static>> {
    let used_str = format_bytes(mem.used);
    let total_str = format_bytes(mem.total);
    let text = format!("{}/{}", used_str, total_str);
    let body_w = width.saturating_sub(5 + text.len() + 1).min(200);
    let ratio = if mem.total > 0 {
        mem.used as f64 / mem.total as f64
    } else {
        0.0
    };
    let color = if ratio > 0.9 {
        Color::Red
    } else if ratio > 0.7 {
        Color::Yellow
    } else {
        Color::Green
    };
    bar_spans("Mem", body_w, ratio, color, text)
}

pub fn render_swap_bar(swp: &SwapData, width: usize) -> Vec<Span<'static>> {
    let used_str = format_bytes(swp.used);
    let total_str = format_bytes(swp.total);
    let text = format!("{}/{}", used_str, total_str);
    let body_w = width.saturating_sub(5 + text.len() + 1).min(200);
    let ratio = if swp.total > 0 {
        swp.used as f64 / swp.total as f64
    } else {
        0.0
    };
    let color = if ratio > 0.9 {
        Color::Red
    } else if ratio > 0.7 {
        Color::Yellow
    } else {
        Color::Green
    };
    bar_spans("Swp", body_w, ratio, color, text)
}
