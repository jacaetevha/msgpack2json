## msgpack2json

msgpack2json is a simple command line tool that translates MessagePack input (from STDIN) to prettified JSON output.

## Usage

1. download the appropriate binary
1. place it in your path
1. pipe some MessagePack goodness into it

```shell
$> echo -e "\x81\xa3\x6d\x73\x67\xa9\x49\x74\x20\x77\x6f\x72\x6b\x73\x21" | msgpack2json
```

... which results in ...

```javascript
{
  "msg": "It works!"
}
```
