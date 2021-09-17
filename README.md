# Colorz !

Why use a boring two tone terminal when you can
have beautiful colors ? That's what i thought.
Colorz is a cli utility that lets you apply colors
on your text. Choose a specific color, or go in
rainbow mode !

## Usage

There are multiple ways you can use this utility,
here are some examples.

### Using stdin

```bash
$ colorz
```

### Using a file

```bash
$ colorz src/main.rs
```

### Using pipes (i konw its stdin, but still)

```bash
$ echo "I love colorz" | colorz
```

### Using the --text flag

```bash
$ colorz --text="I love colorz"
```

## Useful flags

The only useful flag you need to know it the following :

```bash
colorz --help
```

You'll be able to figure out on your own how to use it !
