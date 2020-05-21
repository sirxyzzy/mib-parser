# mib-parser
This project implements a MIB parser, written in pure Rust, this leverages Pest, the elegant parser, see https://pest.rs

MIBs are the way SNMP defines the data exposed by SNMP agents. MIBs are written in ASN.1,
and parsing them is not trivial!

I also have to credit a Java based MIB parser, Mibble, see https://www.mibble.org/ I got a lot of the ideas for the grammar
from the Mibble grammar file.

Right now this project just parses a MIB file, but doesn't generate anything from the parse tree, the next step
for this project is to map the parse tree to a semantic data structure.
