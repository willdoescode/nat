# ⚡️ nat ⚡️

![banner](natbanner.png)

![demo](natdemo.png)

## What is nat?

Nat is a complete replacement for the 'ls' command

Nats features include

- Showing file permissions
- Showing file size
- Showing the date that the file was modified last
- Showing the user that the file belongs to 
- A splash of color to disinguish between files and folders


## Usage

### Installation

```
install the latest release from
https://github.com/willdoescode/nat/releases/

then add the nat file to your path
/usr/local/bin/
```

#### Using nat with ls

in zshrc or bashrc
```bash
alias ls='nat'
```

### Running

```bash
nat <dir>
```

#### Searching for file

```bash
nat <dir (leave empty if in wanted dir)> -f <file>
```

### To edit the code

```bash
git clone https://github.com/willdoescode/nat.git
cd nat
```

## Uninstall steps

```bash
rm /usr/local/bin/nat
```