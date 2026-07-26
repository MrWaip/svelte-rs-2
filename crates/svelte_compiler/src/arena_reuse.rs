use std::cell::RefCell;

use oxc_allocator::Allocator;

const MAX_RETAINED_BYTES: usize = 16 * 1024 * 1024;

thread_local! {
    static RETAINED: RefCell<Option<Allocator>> = const { RefCell::new(None) };
}

pub(crate) fn acquire() -> Allocator {
    RETAINED
        .with(|slot| slot.borrow_mut().take())
        .unwrap_or_default()
}

pub(crate) fn release(mut alloc: Allocator) {
    if alloc.capacity() > MAX_RETAINED_BYTES {
        return;
    }
    alloc.reset();
    RETAINED.with(|slot| {
        *slot.borrow_mut() = Some(alloc);
    });
}
