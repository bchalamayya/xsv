These are some very basic and unscientific benchmarks of various commands
provided by `xsv`. Please see below for more information.

These benchmarks were run with
[worldcitiespop_mil.csv](http://burntsushi.net/stuff/worldcitiespop_mil.csv),
which is a random 1,000,000 row subset of the world city population dataset
from the [Data Science Toolkit](https://github.com/petewarden/dstkdata).

These benchmarks were run on an Intel i7-6900K (8 CPUs, 16 threads) with 64GB
of memory.

```
count                   0.23  seconds  197.88   MB/sec
flatten                 4.41  seconds  10.32    MB/sec
flatten_condensed       4.62  seconds  9.85     MB/sec
frequency               1.96  seconds  23.22    MB/sec
index                   0.21  seconds  216.73   MB/sec
sample_10               0.30  seconds  151.71   MB/sec
sample_1000             0.32  seconds  142.23   MB/sec
sample_100000           0.43  seconds  105.84   MB/sec
search                  0.39  seconds  116.70   MB/sec
select                  0.21  seconds  216.73   MB/sec
sort                    2.94  seconds  15.48    MB/sec
slice_one_middle        0.13  seconds  350.10   MB/sec
slice_one_middle_index  0.01  seconds  4551.36  MB/sec
stats                   1.24  seconds  36.70    MB/sec
stats_index             0.19  seconds  239.54   MB/sec
stats_everything        2.14  seconds  21.26    MB/sec
stats_everything_index  0.99  seconds  45.97    MB/sec
```

### Details

The purpose of these benchmarks is to provide a rough ballpark estimate of how
fast each command is. My hope is that they can also catch significant
performance regressions.

The `count` command can be viewed as a sort of baseline of the fastest possible
command that parses every record in CSV data.

The benchmarks that end with `_index` are run with indexing enabled.
