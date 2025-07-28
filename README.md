# DVPL

A simple tool for compressing and decompressing `.dvpl` files used in **World of Tanks Blitz**.

## Usage

1. Download and extract the executable from the [latest release](https://github.com/lanylow/dvpl/releases).
2. Drag and drop the file or folder you wish to convert onto the executable.
   - Files with a `.dvpl` extension will be **decompressed** and all other files will be **compressed** using the **LZ4HC** algorithm.

## The DVPL format

DVPL files store data that can be either uncompressed or compressed using one of the following algorithms:

- LZ4
- LZ4HC
- Deflate

The file ends with a footer that contains information about the contents. The structure of this footer can be found [here](https://github.com/smile4u/dava.engine/blob/development/Sources/Internal/FileSystem/Private/PackFormatSpec.h#L84).

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](https://github.com/lanylow/dvpl/blob/master/LICENSE) file for details.
