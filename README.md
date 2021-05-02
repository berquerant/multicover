# multicover

Output the set to which each element belongs.

## Usage

### From stdin

``` shell
% (echo 1,2; echo 2,) | multicover
u 10 01 11
1 1 0 0
2 1 1 1
```

### From files

``` shell
% seq 3 > a.log
% seq 2 > b.log
% seq 2 4 > c.log
% multicover a.log b.log c.log
u 100 010 001 110 101 011 111
3 1 0 1 0 1 0 0
1 1 1 0 1 0 0 0
2 1 1 1 1 1 1 1
4 0 0 1 0 0 0 0
```

### Range

``` shell
% multicover a.log b.log c.log -b 2 -d 2
u 110 101 011
4 0 0 0
1 1 0 0
2 1 1 1
3 0 1 0
```

### Decimal index

``` shell
% multicover a.log b.log c.log -i
u 4 2 1 6 5 3 7
2 1 1 1 1 1 1 1
3 1 0 1 0 1 0 0
1 1 1 0 1 0 0 0
4 0 0 1 0 0 0 0
```
