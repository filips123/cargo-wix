# cargo-wix: A cargo subcommand to create a Windows installer using the Wix Toolset

[About](#what-is-cargo-wix) | [Installation](#installation) | [Usage](#usage)  

## What is cargo-wix?

The cargo-wix project is a subcommand for [Cargo](http://doc.crates.io/) that builds a Windows installer (msi) using the [Wix Toolset](http://wixtoolset.org/) from the release build of a binary project. It also supports signing the Windows installer if a code signing certificate is available using the [SignTool](https://msdn.microsoft.com/en-us/library/windows/desktop/aa387764(v=vs.85).aspx) application available in the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk).

## Installation

The cargo-wix project can be installed on any platform supported by the [Rust](https://www.rust-lang.org) programming language, but the [Wix Toolset](http://wixtoolset.org) is Windows only; thus, this project is only useful when installed on a Windows machine. Ensure the following dependencies are installed before proceeding. Note, Cargo is installed automatically when installing the Rust programming language.

- [Cargo](http://doc.crates.io/)
- [Rust](https://www.rust-lang.org/en-US/)
- [WiX Toolset](http://wixtoolset.org)
- [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk), need for signing the installer

The [SignTool](https://msdn.microsoft.com/en-us/library/windows/desktop/aa387764(v=vs.85).aspx) executable is used to optionally sign an installer. It is available as part of the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk).

__Note__, the WiX Toolset compiler (`candle`) and linker (`light`) executables located in the WiX Toolset `bin` folder must be included in the PATH environment variable for this subcommand to work properly. The typical install location for the WiX Toolset `bin` folder is: `C:\Program Files (x86)\WiX Toolset\bin`. Please add this path to the PATH system environment variable.

After installing and configuring the dependencies, execute the following command to install the `cargo-wix` subcommand:

```dos
C:\>cargo install cargo-wix
```

## Usage

__Important__, start and use the Native Build Tools command prompt that was installed with the Windows 10 SDK. This will ensure the `signtool` command is available in the PATH if signing the installer with the cargo-wix subcommand. If not signing the installer, then any command prompt can be used. 

Navigate to the project and run the subcommand:

```dos
C:\Path\To\Project\>cargo wix
```

The `cargo wix` subcommand will search for a `main.wxs` file in a `wix` folder within the root folder of the project. If found, it will compile the `main.wxs` file into a `main.wixobj` file and then link it to create the Windows installer (msi). The installer will be located in the `target\wix` folder. Paths in the `main.wxs` file should be relative to the project's root folder, i.e. the same location as the `Cargo.toml` manifest file.

## License

The `cargo-wix` project is licensed under either the [MIT license](https://opensource.org/licenses/MIT) or [Apache 2.0 license](http://www.apache.org/licenses/LICENSE-2.0). See the [LICENSE](https://github.com/volks73/cargo-wix/blob/master/LICENSE) file for more information about licensing and copyright.
