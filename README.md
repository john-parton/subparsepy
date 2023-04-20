# subparsepy
Python bindings for the rust [subparse](https://docs.rs/subparse/latest/subparse/) library

## Basic usage

```python3

from subparsepy import parse_subtitle

for entry in parse_subtitle("/path/to/subtitle.srt"):
    print(entry.start)  # Start time in milliseconds
    print(entry.end)  # End time in milliseconds
    print(entry.line)  # Text
  ```
