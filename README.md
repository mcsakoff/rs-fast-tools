# FIX/FAST Protocol Utilities

**FAST** (**F**IX **A**dapted for **ST**reaming protocol) is a space and processing efficient encoding method for message oriented data streams.

Uses [fastlib](https://crates.io/crates/fastlib) library.

## `fast-decode`

Decodes FAST protocol messages and prints them out as JSON or plain text.

```
Usage: fast-decode [OPTIONS] [DATA]

Arguments:
  [DATA]  Data file with raw packets

Options:
  -t, --templates <TEMPLATES>    XML file with template definitions
  -o, --output <TEXT_DATA_FILE>  Output file
  -q, --quiet                    Quiet mode; produce no message output
  -j, --json                     Output messages in JSON
  -p, --packet                   Parse messages wrapped in packets
  -h, --help                     Print help
  -V, --version                  Print version
```

## `fast-encode`

Encodes plain text FAST protocol messages (JSON is not supported).

```
Usage: fast-encode [OPTIONS] [DATA]

Arguments:
  [DATA]  Data file with text messages, one message per line

Options:
  -t, --templates <TEMPLATES>   XML file with template definitions
  -o, --output <BIN_DATA_FILE>  Output file
  -p, --packet                  Write messages wrapped in packets
  -h, --help                    Print help
  -V, --version                 Print version
```
