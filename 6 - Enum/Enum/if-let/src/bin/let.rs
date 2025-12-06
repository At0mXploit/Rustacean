let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter is from {state:?}!")
} else {
    count += 1;
}
