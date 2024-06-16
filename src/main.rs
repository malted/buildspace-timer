use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use system_status_bar_macos::*;
use tokio::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let demo_day = NaiveDate::from_ymd_opt(2024, 7, 28)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    spawn(async_infinite_event_loop(time::sleep));

    let mut status_item = StatusItem::new(
        "",
        Menu::new(vec![MenuItem::new(
            "open sage",
            Some(Box::new(|| {
                webbrowser::open("http://sage.buildspace.so").expect("failed to open sage")
            })),
            None,
        )]),
    );

    loop {
        status_item.set_title(message(demo_day));
        time::sleep(time::Duration::from_millis(500)).await;
    }
}

fn message(target: NaiveDateTime) -> String {
    let now: DateTime<Utc> = Utc::now();

    let duration_till_demo_day = target.signed_duration_since(now.naive_utc());
    let total_seconds = duration_till_demo_day.num_milliseconds();
    let days_until_target = total_seconds as f64 / 86400000_f64; // 86400 seconds in a day

    format!("{:.6} days till demo day", days_until_target)
}
