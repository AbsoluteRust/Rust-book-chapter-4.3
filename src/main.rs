fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5, calling the first_word function below 

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    
    let s = String::from("hello");
    
    let s = String::from("hello world");

    let hello = &s[0..5]; //creating a slice using a range within brackets [starting_index..ending_index] , starting is start, but ending is end+1 
    let world = &s[6..11];
    
    let slice = &s[0..2]; //these two slices are equal 
    let slice = &s[..2]; //because ..2 = starting from zero, ending at 1  (because the 2 is end+1)
   
    let len = s.len();

    let slice = &s[3..len]; //same as the above slices, these two are equal
    let slice = &s[3..]; //because the trailing off means "until the last byte"

    let slice = &s[0..len]; //therefore these two are equal,
    let slice = &s[..]; //since they both start at zero, and both end at the end

    let my_string = String::from("hello world");

    //`first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[..6]);
    let word = first_word(&my_string[..]);
    // first word also works on references to `String`s, which are equivelant to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    //first_word works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    //because string literals are string slices already,
    //this works too, without the slice syntax
    let word = first_word(&my_string_literal);

    //array slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; 

    assert_eq!(slice, &[2, 3]);
}

//returns a byte index value into the String parameter
fn first_word(s: &str) -> &str { 
    let bytes =s.as_bytes(); //converting the string into an array of bytes (this is the "as_bytes" method)

    for (i, &item) in bytes.iter().enumerate() { //iterator over the array 
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}