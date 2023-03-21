use std::mem;

pub struct DeferAction<F: FnOnce()> {
    action: Option<F>,
}

impl<F: FnOnce()> DeferAction<F> {
    pub const fn defer(f: F) -> Self {
        Self { action: Some(f) }
    }
}

impl<F> Drop for DeferAction<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        let act = mem::take(&mut self.action).unwrap();
        act();
    }
}
