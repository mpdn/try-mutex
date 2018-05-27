A simple non-blocking mutex (i.e. only `try_lock` is supported), using atomics.

Simpler and faster than the one found in stdlib. Does not support poisoning.

# Performance

This library has criterion benchmarks. Here is a output from running it on my
Ubuntu machine:

    build_try               time:   [24.104 us 24.109 us 24.115 us]
                            change: [-3.1070% -1.3590% +0.0179%] (p = 0.09 > 0.05)
                            No change in performance detected.
    Found 9 outliers among 100 measurements (9.00%)
    4 (4.00%) high mild
    5 (5.00%) high severe

    build_std               time:   [2.0887 ms 2.1055 ms 2.1294 ms]
                            change: [+0.5625% +1.3146% +2.1047%] (p = 0.00 < 0.05)
                            Change within noise threshold.
    Found 5 outliers among 100 measurements (5.00%)
    1 (1.00%) high mild
    4 (4.00%) high severe

    lock_try                time:   [613.61 us 613.77 us 613.98 us]
                            change: [-1.1800% -0.8372% -0.5485%] (p = 0.00 < 0.05)
                            Change within noise threshold.
    Found 9 outliers among 100 measurements (9.00%)
    1 (1.00%) low severe
    8 (8.00%) high severe

    lock_std                time:   [1.5453 ms 1.5459 ms 1.5466 ms]
                            change: [-1.4850% -0.8010% -0.2563%] (p = 0.00 < 0.05)
                            Change within noise threshold.
    Found 11 outliers among 100 measurements (11.00%)
    4 (4.00%) high mild
    7 (7.00%) high severe

    contested_try           time:   [1.0300 ms 1.0312 ms 1.0327 ms]
                            change: [-0.6478% -0.2641% +0.1293%] (p = 0.19 > 0.05)
                            No change in performance detected.
    Found 14 outliers among 100 measurements (14.00%)
    1 (1.00%) low mild
    4 (4.00%) high mild
    9 (9.00%) high severe

    contested_std           time:   [2.3053 ms 2.3079 ms 2.3110 ms]
                            change: [-0.6943% +0.0706% +0.8527%] (p = 0.87 > 0.05)
                            No change in performance detected.
    Found 19 outliers among 100 measurements (19.00%)
    3 (3.00%) high mild
    16 (16.00%) high severe

The gist of it is that, compared to `std::Mutex`, `TryMutex` is approximately 88
times faster to construct and twice as fast to lock. However, performance on
your OS and machine might vary, so be sure to run `cargo bench` yourself.