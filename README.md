# rust-nc

Toy `nc` implemenation in rust, as a learning exercise. Nowhere near feature complete.

Uses [mio](https://github.com/carllerche/mio) (in probably misguided and/or wrong ways).

Currently implemented:

* listen for connection, read/write from STDIN and socket
* connect to listener, read/write from STDIN and socket
* terminate when other side closes connection

Licensed under MIT license. See LICENCE.
