# PNG Secret Message
This CLI util embeds a secret message in a PNG file that can be later sent around and decoded

```
Usage: pngme <COMMAND>

Commands:
  encode (<file_name> <chunk_type> <message>) Encodes a secret message into your PNG. Usage: `pngme encode pic.png ruSt "this is my secret message"`
  decode (<file_name> <chunk_type>) Decodes a secret message from your PNG. Usage: `pngme decode pic.png ruSt`
  remove (<file_name> <chunk_type>) Deletes a secret message from your PNG. Usage: `pngme remove pic.png ruSt`
  print  (<file_name>) Prints all the chunks of your PNG. Usage: pngme print pic.png `
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

See `Learning.md` for information reguarding Chunks and ChunkTypes
