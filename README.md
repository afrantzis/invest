A program to find optimal solutions to investment problems involving options
with profits of the form:

profit(n) = interest * n + bonus, n in [min, max]

The default method used is a dynamic programming solution, but other methods
are available through the `-m, --method` option.

See https://afrantzis.com/posts/dynamic-programming-for-fun-and-profit for more
information and background about this topic.

## Building

Use cargo to build:

    $ cargo build --release

## Running

Use cargo:

    $ cargo run --release -- [options]

or after building you can invoke directly with:

    $ build/target/release/invest [options]

## Options

**-n, --amount \<amount\>** 

[Required] The amount to solve the problem for.

**-o, --opt-file \<optfile\>** [required]

[Required] A file containing the problem options. Each line is an option of the
form: "name min max interest bonus".

**-i, --interest-factor \<interest_factor\>**

A factor to multiply the options' interests by [default: 1.0].

**-m, --method \<method\>**

The method to use to solve the problem [default: dp-iterative-optimized]
[possible values: brute-force, overlapping-subproblems, optimal-substructure,
dp-recursive, dp-recursive- optimized, dp-iterative-optimized, greedy].

## Example

```
$ cat example.opts
opt1 50 99 0.01 0.5
opt2 100 299 0.011 1
opt3 300 499 0.016 2.2
opt4 500 999 0.018 2.5
opt5 1000 1999 0.02 4
```

```
$ invest -n 1720 -o example.opts
opt5: 1020
opt3: 300
opt3: 300
opt2: 100
Value: 40.50000000000001
```
