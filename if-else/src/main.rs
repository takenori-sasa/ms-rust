fn main() {
    // Declare array, initialize all values, compiler infers length = 7
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];
    // Get the first day of the week
    let first = days[0];

    // Get the second day of the week
    let second = days[1];
    let seventh = days[7];
}
