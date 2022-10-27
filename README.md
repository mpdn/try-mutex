A simple non-blocking mutex (i.e. only `try_lock` is supported), using atomics.

Simpler than the one found in stdlib. Does not support poisoning.

This used to be faster than the mutex in the standard library, but benchmarking indicates that
optimizations in the standard library means there is no longer a significant difference
(on my machine). Be sure to run them on your own machine to compare.

Nevertheless, this library may still be useful for embedded or similar cases.
