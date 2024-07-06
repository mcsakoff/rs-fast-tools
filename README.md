# FIX/FAST Protocol Utilities

**FAST** (**F**IX **A**dapted for **ST**reaming protocol) is a space and processing efficient encoding method for message oriented data streams.

Uses [fastlib](https://github.com/mcsakoff/rs-fastlib) library.

## `fast-decode`

Decodes FAST protocol messages and prints them out as JSON or plain text.

### Options

```
-t, --templates <TEMPLATES>  XML file with template definitions
-q, --quiet                  Quiet mode; produce no message output
-j, --json                   Output messages in JSON
-p, --packet                 Parse messages wrapped in packets
-h, --help                   Print help
-V, --version                Print version
```

## TODO

* fast-encode
* sds-client
