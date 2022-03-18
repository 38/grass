use std::io::{Result, Write};
pub(crate) fn write_number<W: Write>(mut fp: W, mut n: i32) -> Result<()> {
    if n == 0 {
        fp.write_all(b"0")
    } else {
        let mut buf = [0; 10];
        let mut offset = 0;
        let mut begin = 0;
        let is_negative = if n < 0 {
            n = -n;
            true
        } else {
            false
        };

        while n > 0 {
            buf[offset] = b'0' + (n % 10) as u8;
            n /= 10;
            offset += 1;
        }
        if is_negative {
            buf[offset] = b'-';
            offset += 1;
        }
        let mut end = offset - 1;
        while begin < end {
            buf.swap(begin, end);
            begin += 1;
            end -= 1;
        }
        fp.write_all(&buf[..offset])
    }
}
