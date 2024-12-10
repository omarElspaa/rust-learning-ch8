fn main() {
    // Rust's standard library includes a number of very useful data structures called collections.
    // Most other data types represent one pecific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data that these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you'll develop over time.

    // We'll discuss how to create and update vectors, strings, and hash maps, as well as what makes each special. To learn about the other kinds of collections provided by the standard library, see the documentation at https://doc.rust-lang.org/sd/collections/index.html
    // Search about manually allocating memory. ( I mean you not the code idiot ) 



    // In a vector (Vec<T>), the generic type T can be any type, including primitive types (i32, f64), custom structs, enums, or even other collections. Rust requires T to be Sized, meaning its size must be known at compile time, because vectors allocate contiguous memory for their elements. If T is not Sized (e.g., trait objects), it must be wrapped in a smart pointer like Box<dyn Trait>. Additionally, T may need to implement specific traits (e.g., Clone, PartialEq) for certain operations, but there are no inherent restrictions on what T can represent.
    // The Vec<T> type in Rust has internal fields that are used for managing its memory.

    //  Length (len): This field represents the number of elements currently stored in the vector. It is stored as a usize value, which is typically 8 bytes on a 64-bit system (since usize is the pointer-size type).

    // Capacity (cap): This field indicates the amount of space (in terms of the number of elements) allocated for the vector on the heap, which may be larger than the len if the vector has been resized. It is also stored as a usize value, typically 8 bytes on a 64-bit system.

    // A regular pointer ( 8 bytes on a 64-bit system ) would just be a memory address, but a fat pointer ( 16 bytes on a 64-bit system ) includes both the address and the size of the data it refers to, making it more flexible for handling dynamically sized types like slices.
    // When you call .as_slice() on a Vec<T>, the Vec<T> is converted into a slice (&[T]), which is a reference to a contiguous block of data. The reason a fat pointer is used is that slices need to track both:
    // A reference to the start of the data (a pointer).
    // The number of elements (the length) in that slice.

    // https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
    //  fat pointer contains a pointer plus some information that makes the DST "complete" (e.g. the length).
    // Slices ([T] and str)
    // The type [T] (for any T) is dynamically sized (so is the special "string slice" type str). That's why you usually only see it as &[T] or &mut [T], i.e. behind a reference. This reference is a so-called "fat pointer".
    // In the case of slices, the additional data that "completes" the DST is simply the length.

    // For sized types, the compiler knows the size and layout of the type at compile time. This means:

    // The reference only needs to store the memory address of the value.
    // The size of the value can be statically determined when generating code, so there's no need to carry size information with the reference.
}