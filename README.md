# urlparams
Given a list of urls returns the unique query parameters appearing in them.

## Usage
```
./urlparams name_of_file.txt
```
## Performance
```
[l@l urlparams]$ wc -l gauairbnbsubs.txt 
5723032 gauairbnbsubs.txt
[l@l urlparams]$ time ./urlparams gauairbnbsubs.txt
...
real	0m3.724s
user	0m3.642s
sys	0m0.081s
```