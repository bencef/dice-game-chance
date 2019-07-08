#[derive(Debug)]
pub struct UsizeToIsizeErr();

pub fn to_isize(v: usize) -> Result<isize, UsizeToIsizeErr> {
    if v > std::isize::MAX as usize {
        Err(UsizeToIsizeErr())
    } else {
        Ok(v as isize)
    }
}
