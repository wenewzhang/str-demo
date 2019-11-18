Basically a String wraps and manages a dynamically allocated str as backing storage.
Since str cannot be resized, String will dynamically allocate/deallocate memory.

A &str is thus a reference directly into the backing storage of the String, while &String is a reference to the “wrapper” object.
Additionaly, &str can be used for substrings, i.e. they are slices. A &String references always the whole string.
