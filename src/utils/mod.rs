pub trait NumOption<T: PartialOrd> {
    fn unwrap_gt_or(self, val: T, default: T) -> T;
}

impl<T: PartialOrd> NumOption<T> for Option<T> {
    fn unwrap_gt_or(self, val: T, default: T) -> T {
        match self {
            Some(v) => {
                if v > val {
                    v
                } else {
                    default
                }
            }
            None => default,
        }
    }
}
