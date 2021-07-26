
pub fn summation(data: &[u32]) -> Option<u32>{
    let mut sum: u32 = 0;
    for num in data.iter() {
        sum = match sum.checked_add(*num){
            Some(r) => r,
            None => return None
        };
    }
    Some(sum)
}