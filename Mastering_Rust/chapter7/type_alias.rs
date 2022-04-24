type  ParserResult<T, E> = Result<ParsedPayload<T>, ParseError<E>>;

pub struct ParsedPayload<T> {
    inner: T
}

pub struct ParseError<E> {
    inner: E
}

pub fn parse_payload<T, E>(stream: &[u8]) -> Result<ParsedPayload<T>, ParseError<E>> {
    unimplemented!();
}

fn main() {
    //TODO
}