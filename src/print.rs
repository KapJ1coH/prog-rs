use chrono::{Local, NaiveDateTime};
use tabled::settings::{Style, Width, object::Segment};
use tabled::{Table, Tabled};
use owo_colors::OwoColorize;

use crate::parser::Task;

// The "View" struct - This defines exactly how the table looks
#[derive(Tabled)]
struct TaskRow {
    name: String,

    weight: String,

    time_left: String,

    visual_bar: String,
}

impl TaskRow {
    fn from_task(task: &Task) -> Self {
        let now = Local::now().naive_local();
        let duration = task.due_date - now;

        // 1. Format Relative Time (e.g., "2d 5h")
        let days = duration.num_days();
        let hours = duration.num_hours() % 24;
        let time_str = if days < 0 {
            "OVERDUE".to_string()
        } else if days == 0 {
            format!("{}h", hours)
        } else {
            format!("{}d {}h", days, hours)
        };
        // Inside from_task...
        let count = days.max(0) as usize;
        let bar = if days <= 2 {
            "■ ".repeat(count).red().to_string() // Red if < 2 days
        } else if days <= 5 {
            "■ ".repeat(count).yellow().to_string() // Yellow if < 5 days
        } else {
            "■ ".repeat(count).green().to_string() // Green otherwise
        };

        Self {
            name: task.name.clone(),
            weight: format!("{:?}", task.weight), // Uses your Debug impl
            time_left: time_str,
            visual_bar: bar,
        }
    }
}

pub fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    let rows: Vec<TaskRow> = tasks.iter().map(TaskRow::from_task).collect();

    let mut table = Table::new(rows);

    table.with(Style::modern());

    println!("{}", table);
}
