__author__ = 'Josh'

import distorm3
import sys

filename = "OUTPOST.EXE"

STATUS_FAILURE = -1
STATUS_SUCCESS = 0

def run():
    # open the file for parsing
    code = None
    try:
        code = open(filename, 'rb').read()
    except Exception as e:
        print "failed to open file"
        return STATUS_FAILURE

    offset = 0
    decode = distorm3.Decode16Bits
    line_count = 0
    lines = []


    iterable = distorm3.DecodeGenerator(offset, code, decode)
    for (offset, size, instruction, hexdump) in iterable:
        line = "%.8x: %-32s %s" % (offset, hexdump, instruction)
        print line
        lines.append(line)
        line_count += 1

    out_file = 'lines.asm'
    out_file_fd = open('lines.asm', 'w')
    for line in lines:
        out_file_fd.write(line)
        out_file_fd.write('\n')
    out_file_fd.close()

    print "total # of lines: {0}".format(line_count)

    return STATUS_SUCCESS

if __name__ == '__main__':
    run()


