# makemkv-renamer
When ripping blu-rays from makemkv its annoying to rename the files one by one. I used to use neovim with some regex to create mv commands to rename them for me. Although that takes a few minutes each time. So I wanted to create a program that I could one line and update the name.

## API
* dir - where the mkv files are located. it defaults to the current directory.
* quality - Includes the bluray, UHD, or DBD markers expected from *arr programs or just general good practice.
* season - Season number. Defaults to 1.
* number - Episode number to start with. Defaults to 1.
For further details use the `-h` flag when running the program.

For an example, running:
```
makemkv-renamer -q bd -s 2 -n 10
```

would rename the files the following way:
| old name | new name |
| -- | -- |
| MyShow Disc 1_t00.mkv | MyShow S02E10 \[BD Remux\]\[1080p\].mkv |
| MyShow Disc 1_t01.mkv | MyShow S02E11 \[BD Remux\]\[1080p\].mkv |
| MyShow Disc 1_t02.mkv | MyShow S02E12 \[BD Remux\]\[1080p\].mkv |

As you can see here, it includes a hardcoded filter for "Disc #" text in the filename.
