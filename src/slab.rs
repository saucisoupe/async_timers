use std::task::Waker;

#[derive(Default)]
pub struct TimerStorage {
    inner: slab::Slab<Timer>,
}

#[derive(Debug)]
pub struct Timer {
    inner: Option<Waker>,
}

impl Timer {
    fn cancel(&mut self) {
        let _ = self.inner.take();
    }

    pub fn wake(self) {
        if let Some(waker) = self.inner {
            waker.wake();
        }
    }
}

impl From<Waker> for Timer {
    fn from(value: Waker) -> Self {
        Self { inner: Some(value) }
    }
}

impl TimerStorage {
    pub(crate) fn insert(&mut self, waker: &Waker) -> usize {
        self.inner.insert(Timer::from(waker.clone()))
    }

    ///this time won't wake up anything
    pub(crate) fn cancel(&mut self, id: usize) {
        unsafe { self.inner.get_unchecked_mut(id).cancel() }
    }

    /// Takes the timer out of storage, returns None if it was cancelled
    pub(crate) fn take(&mut self, id: usize) -> Option<Timer> {
        let timer = self.inner.remove(id);
        if timer.inner.is_some() {
            Some(timer)
        } else {
            None
        }
    }
}
