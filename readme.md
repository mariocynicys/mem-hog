# ?MemoryLeak

This is a reproduction of a case that caused high memory usage that was originally discovered [here](https://github.com/talaia-labs/rust-teos/blob/076c9729130aa78ae90f2b2b2c5b3b7e338791a1/teos/src/watcher.rs#L142-L152).

@fasterthanlime made a video explaining why does this program behave that way: https://www.youtube.com/watch?v=YB6LTaGRQJg
