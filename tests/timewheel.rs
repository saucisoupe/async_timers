mod common;

use async_timers::{DurationTooLong, TimeWheel};
use common::make_waker;
use std::time::Duration;

#[test]
fn test_timer_fires_ms_level() {
    let mut wheel = TimeWheel::new();
    let (counter, waker) = make_waker();

    wheel
        .register_timer(Duration::from_millis(50), &waker)
        .unwrap();

    std::thread::sleep(Duration::from_millis(60));
    wheel.tick();

    assert_eq!(counter.count(), 1);
}

#[test]
fn test_timer_fires_s_level() {
    let mut wheel = TimeWheel::new();
    let (counter, waker) = make_waker();

    wheel
        .register_timer(Duration::from_millis(150), &waker)
        .unwrap();

    std::thread::sleep(Duration::from_millis(200));
    wheel.tick();

    assert_eq!(counter.count(), 1);
}

#[test]
fn test_multiple_timers() {
    let mut wheel = TimeWheel::new();
    let (counter, waker) = make_waker();

    wheel
        .register_timer(Duration::from_millis(20), &waker)
        .unwrap();
    wheel
        .register_timer(Duration::from_millis(40), &waker)
        .unwrap();
    wheel
        .register_timer(Duration::from_millis(60), &waker)
        .unwrap();

    std::thread::sleep(Duration::from_millis(70));
    wheel.tick();

    assert_eq!(counter.count(), 3);
}

#[test]
fn test_cancel_timer() {
    let mut wheel = TimeWheel::new();
    let (counter, waker) = make_waker();

    let id = wheel
        .register_timer(Duration::from_millis(50), &waker)
        .unwrap();
    wheel.cancel_timer(id);

    std::thread::sleep(Duration::from_millis(60));
    wheel.tick();

    assert_eq!(counter.count(), 0);
}

#[test]
fn test_duration_too_long() {
    let mut wheel = TimeWheel::new();
    let (_counter, waker) = make_waker();

    let result = wheel.register_timer(Duration::from_secs(25 * 3600), &waker);
    assert_eq!(result, Err(DurationTooLong));
}

#[test]
fn test_duration_at_limit() {
    let mut wheel = TimeWheel::new();
    let (_counter, waker) = make_waker();

    let result = wheel.register_timer(Duration::from_secs(23 * 3600), &waker);
    assert!(result.is_ok());
}

#[test]
fn test_next_deadline_empty() {
    let wheel = TimeWheel::new();
    assert_eq!(wheel.next_deadline(), None);
}

#[test]
fn test_next_deadline_with_timer() {
    let mut wheel = TimeWheel::new();
    let (_counter, waker) = make_waker();

    wheel
        .register_timer(Duration::from_millis(50), &waker)
        .unwrap();

    let deadline = wheel.next_deadline();
    assert!(deadline.is_some());
    assert!(deadline.unwrap() <= Duration::from_millis(100));
}

#[test]
fn test_timer_not_fired_too_early() {
    let mut wheel = TimeWheel::new();
    let (counter, waker) = make_waker();

    wheel
        .register_timer(Duration::from_millis(100), &waker)
        .unwrap();

    std::thread::sleep(Duration::from_millis(30));
    wheel.tick();

    assert_eq!(counter.count(), 0);
}
