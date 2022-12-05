use color_eyre::eyre::{Context, Result};

type O<I> = Option<<I as Iterator>::Item>;

pub(crate) trait IteratorNextExt: Iterator {
    fn next3(&mut self) -> (O<Self>, O<Self>, O<Self>) {
        (self.next(), self.next(), self.next())
    }

    fn next5(&mut self) -> (O<Self>, O<Self>, O<Self>, O<Self>, O<Self>) {
        (
            self.next(),
            self.next(),
            self.next(),
            self.next(),
            self.next(),
        )
    }

    fn next7(
        &mut self,
    ) -> (
        O<Self>,
        O<Self>,
        O<Self>,
        O<Self>,
        O<Self>,
        O<Self>,
        O<Self>,
    ) {
        (
            self.next(),
            self.next(),
            self.next(),
            self.next(),
            self.next(),
            self.next(),
            self.next(),
        )
    }
}

impl<T> IteratorNextExt for T where T: Iterator {}

pub(crate) trait IntoIteratorExt: IntoIterator {
    fn flat_map_eyre_results<U, M, F>(
        self,
        mut msg: M,
        mut f: F,
    ) -> Result<<Vec<U> as IntoIterator>::IntoIter>
    where
        Self: Sized,
        M: FnMut(usize) -> String,
        F: FnMut(Self::Item) -> Result<U>,
    {
        let r: Result<Vec<U>> = self
            .into_iter()
            .enumerate()
            .map(|(index, v)| f(v).wrap_err_with(|| msg(index)))
            .collect();
        let vec = r?;
        Ok(vec.into_iter())
    }
}

impl<T> IntoIteratorExt for T where T: IntoIterator {}
