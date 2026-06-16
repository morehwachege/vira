# Vira
A lightweight, colorful text search tool inspired by grep. Vira searches for patterns in text and highlights matches with beautiful colored output.

## Features
- 🔍 Pattern Matching - Search for text patterns in files or standard input

- 🎨 Colored Highlights - Matches are displayed in red for easy visibility

- 📁 File Support - Search through text files with ease

- 🔄 Pipeline Friendly - Works seamlessly with Unix pipes and redirections

- 🚀 Fast & Simple - Lightweight and minimal dependencies

## Installation
From Source
```bash
git clone https://github.com/yourusername/vira.git
```
```bash 
cd vira
cargo build --release
sudo cp target/release/vira /usr/local/bin/
```
Using Cargo
```bash
cargo install vira
```
Usage - basic syntax
```bash
vira <pattern> [filename]
```
Examples
Search in a file:

```bash
vira "hello" example.txt
```
Output: Lines containing ```hello``` will be shown with the match highlighted in red.

Search in standard input:

```bash
cat example.txt | vira hello
```

Case-insensitive search:

```bash
 vira "Hello" example.txt  # Matches "hello", "HELLO", "Hello", etc.
```
### Output Example
When you run:

```bash
vira "rust" README.md
```
You might see:

```text
This project is written in rust and is very fast.
The rust ecosystem provides excellent tooling.
```
With "rust" highlighted in ```red``` for better visibility.

## Technical Details
## How It Works
1. Vira reads the pattern from the first command-line argument

2. If a filename is provided, it searches the file line by line

3. If no filename is given, it reads from standard input

4. For each line containing the pattern, it prints the line with matches highlighted in red

5. The search is case-insensitive, but the original text formatting is preserved

## Limitations
Currently supports single pattern matching only

No regex support (planned for future releases)

No line numbering or context display (coming soon)

## Future Plans
- Regular expression support

- Line numbering

- Context lines (show surrounding lines)

- Multiple pattern support (AND/OR logic)

- Recursive directory search

- Configuration file support

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
MIT License - feel free to use this project however you like.

Happy searching!

