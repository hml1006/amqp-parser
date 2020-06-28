use nom::IResult;
use nom::bytes::streaming::tag;
use nom::bytes::streaming::take;
use nom::number::Endianness;

#[inline]
pub fn read_tag<'a>(input: &'a [u8], flag: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    tag(flag)(input)
}

#[inline]
pub fn read_u8(input: &[u8]) -> IResult<&[u8], u8> {
    match take(1usize) (input) {
        Ok((input, ret)) => {
            Ok((input, ret[0]))
        },
        Err(e) => Err(e)
    }
}
