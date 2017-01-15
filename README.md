# Disk Usage Analyzer

Disk Usage Analyzer (DUA) is a linux console application to display what uses the most space in the specified directories. Think about it as
a console version of KDirStat or WinDirStat

# Usage

    USAGE:
        dua [FLAGS] [OPTIONS] [DIR]

    FLAGS:
            --help       Prints help information
        -h               Output sizes in human readable form
        -k               Output sizes in KiBytes
        -V, --version    Prints version information
        -x               no crossing of filesystems

    OPTIONS:
        -n <num>        number of entries to display on each level

    ARGS:
        <DIR>    Directory to analyze
    
## ARGUMENTS

* -h : Display sizes in human readable form
* -k : Display sizes in KiBytes
* -x : Do not descend into different (mounted) filesystems
* -n <num> : Number of entries to display in each directory, default=1

# Examples

    $ dua -h .
    100.0%  34.5M ./
        99.7%  34.4M target/
           74.1%  25.5M debug/
              69.3%  17.7M deps/
                 82.8%  14.6M libclap-c293040e3ef9c3f9.rlib
   

    $ dua -n 2
    100.0%     36187342 ./
        99.7%     36063985 target/
           74.1%     26719018 debug/
              69.3%     18513886 deps/
                 82.8%     15335956 libclap-c293040e3ef9c3f9.rlib
                  5.8%      1073852 libstrsim-65ce24f53c047d6b.rlib
              30.6%      8179760 dua
           25.9%      9344967 release/
              70.0%      6545674 deps/
                 72.8%      4766270 libclap-c293040e3ef9c3f9.rlib
                 11.5%       755728 libunicode_segmentation-c2d11b5e5cccc943.rlib
              29.7%      2773880 dua
         0.1%        44106 src/
           27.9%        12288 .displayer.rs.swp
           27.9%        12288 .main.rs.swp
    
# Author

Pistahh - István Szekeres <szekeres@iii.hu>

