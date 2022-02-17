from sys import argv
from pygrass import BamFile, Bed3 

# This tells GRASS the input should be a BAM file
input = BamFile(argv[1])

# Convert the file format to BED3 and save it
Bed3(input).save_to_file(argv[2])