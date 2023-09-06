# An unvalidated string slice type

Often when dealing with serialized data (from networks or disk), you need to work with strings.
Your specification might say that these strings _should_ be UTF-8, or you might be working with
an older file format that does not require UTF-8, but you expect to find UTF-8 data anyway.

There is a cost to validating UTF-8 strings, in CPU time. Validating also adds new error modes;
what do you do when a legacy file contains a string that is not valid UTF-8?  You could fail to
parse the file entirely, or you could tolerate the error as best as you can.  In some cases, you
just use strings from one data structure to find matching entries in another data structure, so
it is not even important that the strings are not always valid UTF-8.

This crate provides the `RawStr` type, which simply wraps a `&[u8]` value. It provides `Debug` and
`Display` impls, which will display the string the string if possible.

Use `RawStr::from_bytes` to wrap a `&[u8]` value. Use `RawStr::as_bytes` to get the `&[u8]` slice.

This is not meant to be a sophisticated or full-featured crate. It simply serves a niche need.

# Author and acknowledgments

I have listed myself (Arlie Davis (arlie.davis@gmail.com)) as the "author" of this crate, because
`RawStr` is a widely-applicable abstraction. However, I want to acknowledge the influence of the
nearly-identical `RawString` type from the `pdb` crate.  I wrote the equivalent of that type for
some similar work that I am doing, that spans much more than just PDB files, so I thought it would
be best to have a separate crate.
