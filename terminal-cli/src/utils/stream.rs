use std::fmt::Debug;
use std::marker::Send;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

fn debounce_stream_thread<T: Debug + Eq + Clone + Send>(
    input_stream: mpsc::Receiver<T>,
    debounced_tx: mpsc::Sender<T>,
    duration: Duration,
) -> anyhow::Result<()> {
    let mut current_selected: Option<(SystemTime, T)> = None;

    for event in input_stream {
        let previous_event = current_selected.as_ref().is_some();
        let event_equals = current_selected
            .as_ref()
            .map_or(false, |(_, selected_elem)| selected_elem.eq(&event));
        let time_elapsed = current_selected
            .as_ref()
            .map_or(true, |(time, _)| time.elapsed().unwrap() > duration);

        if !previous_event || !event_equals || time_elapsed {
            let current_time = SystemTime::now();
            current_selected = Some((current_time, event.clone()));

            let _ = debounced_tx.send(event);
        }
    }

    Ok(())
}

pub fn debouce_stream<T: Debug + Eq + Clone + Send + 'static>(
    input_stream: mpsc::Receiver<T>,
    duration: Duration,
) -> mpsc::Receiver<T> {
    let (debounced_tx, debounced_rx) = mpsc::channel::<T>();
    thread::spawn(move || debounce_stream_thread(input_stream, debounced_tx, duration));

    debounced_rx
}
