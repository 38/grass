/*
#!/usr/bin/python3

import sys
import random

n = int(sys.argv[1])

def generate_next_start(start, end, k):
    linear_p = random.random()
    adjusted = 1 - linear_p ** (1.0 / k)
    return start + int((end - start) * adjusted)

begin = 0
end = 3000000000

for k in range(0, n):
    begin = generate_next_start(begin, 3000000000, n)
    n -= 1
    c = str(int(begin / 1000000000) + 1)
    b = begin
    e = b + random.randint(100, 100000)
    print(c,b,e)
*/
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

double generate_next_start(unsigned start, unsigned end, unsigned k) {
	double linear_p = rand() / (double)RAND_MAX;
	double adjusted = 1 - powf(linear_p, 1.0 / k);
	return start + (end - start) * adjusted;
}

int main(int argc, char** argv) {
	srand((unsigned)time(NULL));
	unsigned begin = 0;
	unsigned end = 300000000;

	unsigned k = atoi(argv[1]);

	for(;k > 0;k --) {
		begin = generate_next_start(begin, end, k);
		unsigned chr = begin / 100000000 + 1;
		unsigned e = begin + 100 + 100000 * rand() / (double)RAND_MAX;
		printf("%d\t%u\t%u\n", chr, begin, e);
	}
	return 0;
}
