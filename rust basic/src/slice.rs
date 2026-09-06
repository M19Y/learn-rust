/* Slice
*
*
*/

fn main() {
    let array: [u8; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // Get the array by range slice
    // Range -> (start..end)
    // Range From -> (start..)
    // Range Full -> (..)
    // Range Inclusive -> (start..=end)
    // Range To -> (..end)
    // Range To Incluseve -> (..=end)

    // get a full range reference array
    let full_range_slice: &[u8] = &array[..];
    println!("Full range array slice  = {:?}", full_range_slice);

    let range: &[u8] = &array[0..10];
    println!("range array slice  = {:?}", range);

    let from_range: &[u8] = &array[5..];
    println!("from range array slice  = {:?}", from_range);

    let inclusive_range: &[u8] = &array[5..=7];
    println!("from range array slice  = {:?}", inclusive_range);
}
