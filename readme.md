# MorseyType - Morse Code Typing Trainer

MorseyType is a terminal-based Morse code typing trainer designed to help you improve your Morse code proficiency through structured practice sessions. The application presents words from english vocabulary lists, challenges you to type them in Morse code, and provides real-time performance feedback.

## Features

- ⏱ Real-time performance tracking with WPM (Words Per Minute) calculation
- 🌍 Support for english language
- 📊 Dynamic Morse code reference table for quick lookup
- ⚙️ Adjustable threshold for dot/dash distinction
- 📈 Real-time feedback on your Morse input and decoding

## Installation

1. Clone the repository:

```bash
git clone https://github.com/piter231/morseytype.git
cd morseytype
```

2. Build the application:

```bash
cargo build --release
```

## Usage

### Basic Command

```bash
cargo run -- [WORD_COUNT] [THRESHOLD_MS]
```

### Examples

```bash
# 10 words, 150ms threshold
cargo run

# 5 words, 200ms threshold
cargo run -- 5 200
```

### Parameters

| Parameter    | Default | Description                                                               |
| ------------ | ------- | ------------------------------------------------------------------------- |
| WORD_COUNT   | `10`    | Number of words to practice                                               |
| THRESHOLD_MS | `150`   | Duration threshold in milliseconds to distinguish dots (.) and dashes (-) |

### Key Bindings

| Key          | Function                                                        |
| ------------ | --------------------------------------------------------------- |
| `Space`      | Press and release quickly for dot (.), hold longer for dash (-) |
| `f`          | Insert letter separator (/)                                     |
| `j`          | Insert word separator (space)                                   |
| `;`          | Backspace (remove last character)                               |
| `q` or `Esc` | Exit program                                                    |

## Word List Credits

Special thanks to [SMenigat](https://github.com/SMenigat) for creating and maintaining the [thousand-most-common-words](https://github.com/SMenigat/thousand-most-common-words) repository, which provides the word lists used in this application. I converted the en.json from this repo into current dict.rs, in order to make it easy to use as one execuitable.

## Technical Notes

1. **Input Handling**: The application uses precise timing to distinguish between dots (.) and dashes (-). The default threshold is 150ms, but you can adjust this based on your typing speed and preference.

2. **Performance**: The real-time WPM calculation is based on completed words. The final average WPM is calculated based on the total time taken to complete all words.

## Development Journey

Developing MorseyType was a challenging but rewarding experience. One of the most difficult aspects was implementing the precise timing mechanism for distinguishing between dots and dashes. After several iterations of trial and error with different timing approaches, I finally succeeded in creating a reliable system that:

1. Accurately measures key press duration
2. Provides consistent feedback
3. Handles system interrupts gracefully
4. Maintains performance across different platforms

The result is a responsive typing experience that faithfully translates your key presses into Morse code characters.

## Contribution

Contributions are welcome! If you'd like to improve MorseyType, please follow these steps:

1. Fork the repository
2. Create a new branch for your feature (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
