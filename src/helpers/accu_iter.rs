pub struct AccuIter<Accu, Func, Res>
where
    Func: Fn(&Accu) -> (Accu, Option<Res>),
{
    accu: Accu,
    func: Func,
}

impl<Accu, Func, Res> Iterator for AccuIter<Accu, Func, Res>
where
    Func: Fn(&Accu) -> (Accu, Option<Res>),
{
    type Item = Res;

    fn next(&mut self) -> Option<Self::Item> {
        let (accu, result) = (self.func)(&self.accu);
        self.accu = accu;
        result
    }
}

#[allow(dead_code)]
pub fn accu_iter<Accu, Func, Res>(initial: Accu, f: Func) -> AccuIter<Accu, Func, Res>
where
    Func: Fn(&Accu) -> (Accu, Option<Res>),
{
    AccuIter {
        accu: initial,
        func: f,
    }
}
