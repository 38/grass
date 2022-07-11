import gzip
import pathlib

def detect_gzip(path : str) -> bool:
    """
    Detect if the file is gzipped.
    """
    fp = open(path, "rb")
    magic = fp.read(2)
    return magic == b"\x1f\x8b"

def detect_xam(path : str) -> str:
    """
    Detect if the file is a BAM, CRAM, or SAM file.
    """
    fp = open(path, "rb")
    head = fp.read(4)
    if head == b"CRAM":
        return "cram"
    elif detect_gzip(path):
        fp = gzip.GzipFile(path)
        head = fp.read(4)
        if head == b"BAM\x01":
            return "bam"
    return "unknown"

def detect_uncompressed_text_format(reader, arg_bag = None) -> str:
    """
    Detect the file format of the given uncompressed text file.
    """
    detected_category = "none"
    for line in reader:
        if type(line) != str:
            line = line.decode("utf8")
        if detected_category == "none":
            if line.startswith("##fileformat=VCF"):
                return "vcf"
            elif line.startswith("#"):
                detected_category = "bed"
            elif line.startswith(":") or line.startswith(">"):
                detected_category = "fasta"
            elif line.startswith("@"):
                detected_category = "sam"
            else:
                detected_category = "bed"
        if detected_category != "bed":
            return detected_category
        if not line.startswith("#"):
            if arg_bag != None:
                arg_bag["num_of_fields"] = len(line.split("\t"))
            return detected_category
    return "unknown" 

def detect_file_format(path : str, arg_bag = None) -> str:
    """
    Detect the file format of the given file.
    arg_bag is a dictionary that can be used to pass extra information of the file format.
    """
    arg_bag = dict() if arg_bag == None else arg_bag
    try:
        xam = detect_xam(path)
        if xam != "unknown":
            return xam
        fp = None
        if detect_gzip(path):
            arg_bag["compressed"] = True
            fp = gzip.GzipFile(path)
        else:
            fp = open(path)
        return detect_uncompressed_text_format(fp, arg_bag)
    except Exception:
        path = pathlib.Path(path)
        components = path.name.split('.')
        if components[-1] == 'gz' and len(components) > 1:
            arg_bag['compressed'] = True
            components = components[:-1]
        if components[-1] == 'bed' and len(components) > 1:
            arg_bag['num_of_fields'] = 3
            return "bed"
        elif components[-1] == 'vcf' and len(components) > 1:
            return "vcf"
        return "unknown"