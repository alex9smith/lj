# lj

Like `ls` but outputs JSON!

## Usage

```bash
$ lj [path] [options]
```

### Options
```
--help              Shows the available options.

-r, --recursive     If set, lj will recursively search directories at the target path for files
                    up to a maximum search depth. Set the depth with '-d=', defaults to 5.
                    
-d, --depth         The search depth to use when recursively searching folders for files.
                    If set, you must also pass '-r', Defaults to 5.   
```

## Development

`lj` is a Rust binary. Install [Cargo]() for your OS, clone this repo and run `cargo build` from the root.
