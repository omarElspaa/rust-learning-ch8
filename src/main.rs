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



    // Vector data in Rust are stored on the heap, and unlike arrays, they are dynamically allocated and can grow in size. When a vector needs to reallocate memory, it typically does so by allocating a new, larger memory block rather than expanding the existing one. Reallocation only occurs when the vector's current capacity is filled, and an attempt is made to add a new element. This process helps maintain efficient memory usage while ensuring that elements remain in a contiguous block.



    // In both C and Rust, the compiler inserts padding to ensure that fields within structs are aligned to their required memory boundaries, based on their size or system-defined alignment requirements. Even if a field like char doesn’t have strict alignment, padding is added after it to ensure that larger fields (like int) are properly aligned, which is important for performance and correctness. This padding ensures efficient memory access, prevents misalignment errors, and maintains a consistent layout across different platforms.



    let _v: Vec<i32> = Vec::new(); // To create a new empty vector.
    // Note that we added a type annotation here. Becauses we aren't interting any values into this vector, Rust doesn't know what kind of elements we intend to store. This is an important point. Vectors are implemented using generics; we'll cover how to use generics with your own types in Chapter 10. For now, know that the Vec<T> type provided by the standard library can hold any type. When we create a vector to hold a specific type, we can specify the type within angle bracket

    let _v2 = vec![1, 2, 3]; // Creates a new Vec<i32> that holds the values 1, 2, and 3. The integer type is i32 because that's the default inteer type.
    // Because we've given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn't necessary.

    // For some reason if the vector is initialized empty it points to 0x1 or 0x2 or 0x4 or 0x8 ( There might be multiple vectors pointing to the same location ), it differs depending on the type of T, I suspect that this is due to memory alignment, but I don't know if these values are manually specified in rust source code.

    let mut _v3 = Vec::new();
    _v3.push(1); // As with any varible, if we want to be able to change its value, we need to make it mutable using the mut keyword.
    // The number 1 is of type i32, and Rust infers this from the data,so we don't need the Vec<i32> annotation.


    // To read elements of Vectors:
    // v[index]
    // v.get(index) => Option<&T>

    // For more on the implementation details of the Vec<T> type, see "The Rustonomicon" at https://doc.rust-lang.org/nomicon/vec/vec.html.

    // Iterating:

    // let v = vec![100, 32, 57];
    // for i in &v {
    //  println!("{i}");
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //  *i += 50;
    // }

    // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator. We'll talk more about the dereference operator in "Following the Pointer to the Value" on page 322.

    let mut s = String::new(); // Creating a new, empty string
    let s2 = "initial contents".to_string();
    let mut s3 = String::from("initial contents");
    s3.push_str("IDK");
}