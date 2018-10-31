// HTTP2 preface is used to detect clients that want to upgrade the socket
// const PREFACE: [u8; 24] = *b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

// Trailing ETag value should be removed in favor of appending a generated value
// const TWO_O_TWO: [u8; 59] = *b"HTTP/1.1 202 OK\r\nContent-Length=20\r\nETag=----------------\r\n";
