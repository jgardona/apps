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
+------+--------------------+
| kind | name               |
+------+--------------------+
| ✔    | texstudio.desktop  |
+------+--------------------+
| ✔    | SimulIde.desktop   |
+------+--------------------+
| ✔    | Stellarium.desktop |
+------+--------------------+
| ✔    | krita.desktop      |
+------+--------------------+
| ✔    | inkscape.desktop   |
+------+--------------------+
```

### Count

```
$ apps read --count
Number of local applications: 5
```

### Tests

The tests cant run on multithread environment, because the **concurrent file acceses will break the process**. Then use the test.sh script in folder.