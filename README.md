It is just an attempt in writing a Xbox gamepad driver for the Linux Kernel in Rust.
I used as reference the current (6.13) upstream kernel driver "Xpad" that was wrote in C
I would like to thanks for all the people that contributed to the aforementioned driver.

As of today there is no official support for Rust code in the Kernel,
because of that, even if the code had matured enough to be even buildable, 
In fact it's not remotely buildable lacking even the Cargo.toml,
needless to say it can not be used even more, can not be commited to the Kernel
The purpose to this code is only for education purpose at the moment,
There are several issues in this code that I may fix later or not

Commits are welcomed!
