# Temperature Converter

A simple Rust command-line program for converting temperatures between Celsius and Fahrenheit. This project provides a basic interactive interface where users can choose between two conversion options and input a temperature to convert.

## Features

- **Convert Celsius to Fahrenheit**
- **Convert Fahrenheit to Celsius**
- **Interactive CLI**: User-friendly command-line prompts to select options and input temperature values.
- **Error Handling**: Handles invalid inputs and prompts the user to re-enter values if the input is not valid.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) must be installed.

### Running the Program

1. Clone this repository:

    ```sh
    git clone https://github.com/imaazkhalid/temperature_unit_converter.git
    cd temperature_unit_converter
    ```

2. Run the program with `cargo`:

    ```sh
    cargo run
    ```

3. Follow the on-screen instructions to select a conversion option and enter a temperature value.

## Usage

Once the program is running, you will see a menu like this:

```plaintext
Select an option.
(1) - Convert Celsius to Fahrenheit
(2) - Convert Fahrenheit to Celsius
```

- Enter `1` to convert Celsius to Fahrenheit, or `2` to convert Fahrenheit to Celsius.
- After selecting an option, enter the temperature you want to convert when prompted.
- The program will then display the converted temperature.

### Example

To convert `25°C` to Fahrenheit:

```plaintext
Select an option.
(1) - Convert Celsius to Fahrenheit
(2) - Convert Fahrenheit to Celsius
1
Enter temperature in Celsius
25
Temperature in Fahrenheit: 77
```

To convert `77°F` to Celsius:

```plaintext
Select an option.
(1) - Convert Celsius to Fahrenheit
(2) - Convert Fahrenheit to Celsius
2
Enter temperature in Fahrenheit
77
Temperature in Celsius: 25
```

## Project Structure

- **`src/main.rs`**: Contains all logic for the program, including user input handling, conversion functions, and display.

## Conversion Formulas

- **Celsius to Fahrenheit**: `°F = (°C × 9/5) + 32`
- **Fahrenheit to Celsius**: `°C = (°F - 32) × 5/9`

---

This project is a beginner-level implementation and is intended as a practice exercise for learning Rust basics, especially standard input/output, loops, and basic error handling. Feel free to clone and modify as you learn more about Rust!
