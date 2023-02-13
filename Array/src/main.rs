/* //Array
fn main() {
    let a = [1, 2, 3, 4, 5];
    let x: [u32; 5] = [1, 2, 3, 4, 5];
    let y:[3; 5] //equal to let y = [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];
}
*/

 //Tuple
fn main(){
    let a = (1, 6.4, 500);
    let x: (u8, f64, i32) = (1, 6.4, 500);

    let first = a.0;
    let second = a.1;
}