use std::{cell::RefCell, rc::Rc};

use crate::prelude::{PlatformEvents, Renderer};

pub trait Component<R, P>
where
    R: Renderer<P>,
    P: PlatformEvents,
{
    type State: ComponentState<R, P>;

    fn create(ui: R, props: Self) -> Self::State;
}

pub trait ComponentState<R, P>
where
    R: Renderer<P>,
    P: PlatformEvents,
{
    fn roots(&self) -> Vec<u32>;
}

impl<R, P, C> ComponentState<R, P> for Rc<RefCell<C>>
where
    C: ComponentState<R, P>,
    R: Renderer<P>,
    P: PlatformEvents,
{
    fn roots(&self) -> Vec<u32> {
        self.borrow().roots()
    }
}