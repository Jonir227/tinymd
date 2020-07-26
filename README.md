# Rust로 미니 마크다운 컴파일러 만들기

## 출처
https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/

## TODO

[] At the end of the match first_char.pop() block, we have to repeat ourselves by checking whether the _ptag or _htag are open. How could you encapsulate this logic in a function to better adhere to the DRY principle?

[] The way we are opening and reading the input file is not the same as what you find in the Rust docs for read_lines here. How (and why) is the linked Rust code different?

[] How could you add in some other HTML tags at the top of your output file, like a <style> tag or even a link to a CSS file before you iterate over each line from the input file?

[] How would you add support for more than one character as the flag? For example, how could you parse ## This is a second-order heading? (Hint: the first_char is only the first character of line_contents because we used .take(1)…)

[] Try to add support for <em> (asterisks) and <strong> (double asterisks)

[] When it comes to error checking, Rust has evolved over time to be less verbose. Try out some of the ways Rust lets you skip the verbosity in
error-handing by updating parts of the code we wrote to manually unwrap Result objects.

[] tutorial hasn’t even touched on one of the coolest parts of Cargo–integrated testing built right into the ecosystem! Challenge yourself to use some TDD practices on your next tinker project in Rust.