// We have learnt a lot of things in this topic
// To put things simply in the memry of the computer stack memory holds all the variables with
// known size whereas heap holds the dynamic sized memory

fn min(s : String){

}
fn main() {
    let s = "hello"; // created on the stack as immutable
    let mut string = "World".to_string(); // created on heap as mutable
    println!("{}", string);
    min(string);
    //????????println!("{}", string);// This gives an error because the ownership is changed
    let s1 = "Hello".to_string();
    let s2 = s1;
    //????????println!("{}", s1); // This gives an error as we tried to get to a memory which is alrady moved elsewhere
    // To solve the above error we can use let s2 = s1.clone() copies the contents to the new var
}
