use active_win_pos_rs::get_active_window;
use chrono::Local;
use colored::*;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    println!("{}", "=== Lazy Monitor Pro: Running ===".green().bold());
    println!("📊 Real-time data writing: log.csv");
    println!("📈 Visual report: report.html (double-click to open)");
    println!("❌ Exit method: Press Ctrl + C \n");

    // 1. Initialize CSV
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.csv")
        .expect("Failed to open log.csv");

    if file.metadata().unwrap().len() == 0 {
        writeln!(file, "Timestamp,AppName,Duration").unwrap();
    }

    // 2. State Tracking
    let mut last_app_name = String::from("None");
    let mut start_time = Instant::now();
    
    // 3. In-memory statistics table (App name -> total seconds)
    let mut stats: HashMap<String, u64> = HashMap::new();

    loop {
        let current_app_name = match get_active_window() {
            Ok(window) => window.app_name,
            Err(_) => "System/Idle".to_string(),
        };

        if current_app_name != last_app_name {
            let duration = start_time.elapsed();

            if duration.as_secs() > 1 && last_app_name != "None" {
                let secs = duration.as_secs();
                let now_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                // --- A. Terminal Print ---
                println!("[{}] {} -> {}s", now_str.dimmed(), last_app_name.yellow(), secs);

                // --- B. Write to CSV ---
                writeln!(file, "{},{},{}", now_str, last_app_name, secs).unwrap();

                // --- C. Update in-memory statistics & generate HTML report ---
                // Add time to the corresponding App
                *stats.entry(last_app_name.clone()).or_insert(0) += secs;
                
                // Every time data updates, rewrite the HTML file
                generate_html_report(&stats);
            }

            last_app_name = current_app_name.clone();
            start_time = Instant::now();
        }

        sleep(Duration::from_secs(1));
    }
}

// This is a "magic function" that stuffs Rust data into an HTML template
fn generate_html_report(stats: &HashMap<String, u64>) {
    let mut labels = String::new();
    let mut data = String::new();

    // Convert the data in the HashMap into JS array format
    for (app, duration) in stats {
        labels.push_str(&format!("'{}',", app));
        data.push_str(&format!("{},", duration));
    }

    // This is a hardcoded HTML template that includes Chart.js
    let html_content = format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>My Lazy Monitor</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: sans-serif; text-align: center; padding: 50px; background: #f4f4f9; }}
        .container {{ width: 60%; margin: auto; background: white; padding: 20px; border-radius: 10px; box-shadow: 0 0 10px rgba(0,0,0,0.1); }}
    </style>
</head>
<body>
    <div class="container">
        <h2>💻 App Usage Time Distribution (seconds)</h2>
        <canvas id="myChart"></canvas>
    </div>
    <script>
        const ctx = document.getElementById('myChart');
        new Chart(ctx, {{
            type: 'doughnut',
            data: {{
                labels: [{labels}],
                datasets: [{{
                    label: 'Usage Time (seconds)',
                    data: [{data}], 
                    borderWidth: 1
                }}]
            }}
        }});
    </script>
</body>
</html>
"#,
        labels = labels,
        data = data
    );

    // Overwrite report.html
    fs::write("report.html", html_content).expect("Unable to generate the report");
}