# 🌡️ Rust Temperature Converter

A simple, interactive CLI app to convert between Celsius and Fahrenheit, written in Rust.  
Great for beginners learning Rust fundamentals like:

- `match` statements
- `std::io` for user input/output
- Parsing strings into numbers
- Formatting floating-point output
- Working with functions and string slices

---

## 🚀 Features

- ✅ Convert Celsius to Fahrenheit
- ✅ Convert Fahrenheit to Celsius
- ✅ Interactive CLI with clear prompts
- ✅ Rounded output to 2 decimal places
- ✅ "Press any key to exit" pause at the end

---

## 🖥️ Demo

```bash
Welcome to temperature converter

    1) Celsius to Fahrenheit
    2) Fahrenheit to Celsius

Choose the option which you want: 1
Enter temperature in Celsius: 34
34.00°C is 93.20°F

Press any key to exit...
```

---

## 📦 How to Run

1. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).
2. Clone this repo or copy the `.rs` file.
3. In the project directory, run:

```bash
cargo run
```

Or, if it's a single file:

```bash
rustc main.rs && ./main
```

Also you can build it for windows and linux using (configured for compiling program in linux):

```bash
chmod +x build_release.sh
./build_release.sh
```

---

## 🧠 What You’ll Learn

This project is great if you're learning Rust and want hands-on practice with:

* `io::stdin()` and `io::stdout()`
* `flush()` to force prompt output
* Handling and parsing user input
* `match` and `trim()`
* Floating point math and output formatting

---

## 🛠️ Improvements You Could Try

Want to push yourself further?

* [ ] Add a loop to allow multiple conversions
* [ ] Add error handling (e.g. handle `NaN`, empty input)
* [ ] Use colors in terminal using the `colored` or `crossterm` crate
* [ ] Support command-line arguments using `clap`

---

## 🦀 Built With

* [Rust](https://www.rust-lang.org/) – systems programming language focused on safety and performance

---

## 📄 License

This is a practice project, feel free to use or share it however you'd like.