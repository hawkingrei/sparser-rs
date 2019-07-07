use time::precise_time_ns;

#[inline(always)]
pub fn time_start() -> u64 {
    precise_time_ns()
}

#[inline(always)]
pub fn time_stop(start: u64) -> u64 {
    let end = precise_time_ns();
    return (end - start) / 1000;
}
