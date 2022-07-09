#!/bin/bash
gcc -o generate-random-bedfile generate-random-bedfile.c -O3 -lm

for ((SIZE=1000000; SIZE < 20000000; SIZE += 2000000))
do
	./generate-random-bedfile ${SIZE} > a.bed
	./generate-random-bedfile ${SIZE} > b.bed
	/usr/bin/time -o grass_time.txt -f "%e" ./intersect-and-expand.py a.bed b.bed > /dev/null
	/usr/bin/time -o bedtools_time.txt -f "%e" ./intersect-and-expand-bedtools.sh a.bed b.bed > /dev/null
	/usr/bin/time -o grass_time.2.txt -f "%e" ./intersect.py a.bed b.bed > /dev/null
	/usr/bin/time -o bedtools_time.2.txt -f "%e" ./intersect-bedtools.sh a.bed b.bed > /dev/null
	GRASS_EXP_TIME=$(cat grass_time.txt)
	BEDTOOLS_EXP_TIME=$(cat bedtools_time.txt)
	GRASS_INT_TIME=$(cat grass_time.2.txt)
	BEDTOOLS_INT_TIME=$(cat bedtools_time.2.txt)
	rm {a,b}.bed {grass,bedtools}_time*.txt
	printf "%d\t%f\t%f\t%f\t%f\n" ${SIZE} ${GRASS_EXP_TIME} ${BEDTOOLS_EXP_TIME} ${GRASS_INT_TIME} ${BEDTOOLS_INT_TIME}
done
