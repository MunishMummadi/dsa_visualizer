use std::time::Instant;

pub fn measure_execution_time<F>(func: F) -> u128
where
    F: FnOnce(),
{
    let start = Instant::now();
    func();
    let duration = start.elapsed();
    duration.as_millis()
}

// TODO: Create a UI component to display the measured execution time.