# Apps
Manage your local application folder. You can list, count, create and remove desktop launcher files.


### Install

```
cargo install apps
```

### Usage

```
$ apps -h
Helps you to create application launchers in gnome based desktops

Usage: apps <COMMAND>

Commands:
  read    Reads data from application's folder
  create  Creates a new application launcher
  remove  Removes an application launcher from folder
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### List

```
$ apps read --list
+--------------------+
| name               |
+--------------------+
| texstudio.desktop  |
+--------------------+
| krita.desktop      |
+--------------------+
| inkscape.desktop   |
+--------------------+
```

### Create
```
$ apps create "example" "example.icon" "executable" "example comment"

$ apps read -l
+-------------------+
| name              |
+-------------------+
| inkscape.desktop  |
+-------------------+
| krita.desktop     |
+-------------------+
| texstudio.desktop |
+-------------------+
| example.desktop   |
+-------------------+
```

### Remove
```
$ apps remove example
Application removed

$ apps read -l
+-------------------+
| name              |
+-------------------+
| inkscape.desktop  |
+-------------------+
| krita.desktop     |
+-------------------+
| texstudio.desktop |
+-------------------+
```
### Count

```
$ apps read --count
Number of local applications: 5
```
### Tests

The tests cant run on multithread environment, because the **concurrent file access will break the process**, then use the test.sh script in folder.
