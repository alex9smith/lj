# lj

Like `ls` but outputs JSON!

## Installation

`lj` is available on Homebrew for Intel Mac users.
```bash
brew tap alex9smith/lj
brew install lj
```
For all other OSs and architectures, you'll need to build from source and copy the binary to 
your favourite folder on your $PATH.

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

Pull requests with reasonable changes are very welcome. If you have a feature request, please [open an issue](https://github.com/alex9smith/lj/issues) on Github.