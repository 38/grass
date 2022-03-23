#!/bin/bash
 
bedtools intersect -sorted -a $1 -b $2 | bedtools slop -b 5 -i - -g g.txt
