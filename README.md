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

### Using pipes (i know its stdin, but still)

```bash
$ echo "I love colorz" | colorz
```

### Using the --text flag

```bash
$ colorz --text="I love colorz"
```

### Useful flags

The only useful flag you need to know it the following :

```bash
colorz --help
```

You'll be able to figure out on your own how to use it !

## Decisions

### Why HSV ?

Initially, the program was intended to be a lolcat copy but in rust. HSV is the representation of a color as a hue, a saturation and a value. This made it easy to only increment the hue and get the rainbow effect.

### Why RGB ?

Since the RGB notation is one of the most used worldwide, i thought it would be useful and relevant to have the ability to use the `#[a-f0-9]{6}` (e.g : `#38afff`) format to give a color to the program. In the current state of the program, the way RGB support works is that the color gets immediately converted to HSV.
