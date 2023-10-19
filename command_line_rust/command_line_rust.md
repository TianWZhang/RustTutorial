for Cargo to not print status messages about compiling and running the code
cargo run --quiet 

Testing takes advantage of concurrency to run many tests in parallel, so the test results may appear in a different order each time we run them.
cargo test -- --test-threads=1

Cargo places the downloaded source code into .cargo in the home directory, and the build artifacts go into the target/debug/deps directory of the project. Each program we build can use different versions of crates, and each program is built in a separate directory.

du -shc . 
the project now weights in at about 80MB

It's dangerous to assume that each fallible call will succeed.
Before programming in Rust, I'd only ever considered one amorphous idea of computer memory. Having studiously avoided languages that required me to allocate and free memory, I was only vaguely aware of the efforts that dynamic languages make to hide these complexities from me.

set aside a chunk of memory

Use ? to unpack an Ok value or progagate an Err.

Predicates allow us to execute an external command from within a Rust program, check the exit status, and verify the contents of both STDOUT and STDERR.

file tests/inputs/*.txt
Use the file command to report file type information.

``` rust
let mut contents = String::new();
//may crash the computer if the file's size exceeds tje amount of memory
file.read_to_string(&mut contents).unwrap(); 
let bytes = contents.as_bytes();
//more dangerous: num_bytes might be larger than the size of bytes
print!("{}", String::from_utf8_lossy(&bytes[..num_bytes]));
```

wc xxx tests/inputs/fox.txt 2>err
redirect STDERR to the file err

Don't take references to values from a struct that goes out of scope at the end of the function and is then dropped. Returning a struct that stores references to a dropped value would lead to dangling pointers.
The Rust compiler would like to know exactly how long the values are expected to stick around relative to one another.

cargo run -- - < tests/inputs/one.txt

find . -type f/l/d
find . -name "*.csv" -o -name ".txt" # -o = or
find . -name "*.csv" -type f -o -type l 
find .  \( -type f -o -type l \) -name "*.csv"
find a/b d -name "*.mp3"
file . -size 0 -delete

separate multiple optional values from the multiple positional values

In regex syntax, . is a metacharacter that means any one character and the asterisk means zero or more of the previous character. So .* is the equivalent regex of * in a file glob. 
Use \ to escape the literal dot, and \ must itself be backslash-escaped on the command line.
.*\\.txt = .*[.]txt
".*[.]csv$"      ^foo
The caret(^) anchors the pattern to the beginning of the search string and $ anchors the expression to the end.

Being enclosed in quotes indicates that this comma is not a field delimiter, which is a way to escape the delimiter. The parsing of delimited text files should respect escaped delimiters.

grep -i pattern fox.txt
grep -v|--invert-match pattern fox.txt
grep -c|--count pattern fox.txt
grep -r pattern .
cat * | grep -i the

comm -12 <(sort cities1.txt) <(sort cities2.txt)
sort cities2.txt | comm -12 <(sort cities1.txt) -
comm file1.txt file2.txt | sed "s/\t/--->/g" #replace each \t with --->

 tail -n 4 tests/inputs/ten.txt
 tail -c 8 tests/inputs/ten.txt | cat -e #six byte-sized characters and two byte-sized newline characters  pipe the output to cat -e to display the $ at the end of each file
 tail -n +8 tests/inputs/ten.txt #start printing from line 8
 time tail tests/inputs/1M.txt > /dev/null 

cargo build --release
cargo install hyperfine
 hyperfine -i -L prg tail,target/release/l11_tailr '{prg} 1M.txt > /dev/null'
 hyperfine -i -L prg tail,target/release/l11_tailr '{prg} -n 100000 1M.txt > /dev/null'

 You will nip the problem in the bud if you return the files in a consistent, sorted order.

Write the following to [cargo project]/.git/hooks/pre-commit
 ```
 cargo fmt
 exec cargo clippy -- -D warnings #turn warnings into errors
 ```
 chmod a+x .git/hooks/pre-commit

 cal -m 9 2023 | cat -e

 std::fs::remove_file(path)?


 cargo doc --open --document-private-items

 If we're going to build reliable and efficient software like the Rust motto claims, it is incumbent on us to shoulder this burden.
 
 
```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config {query, file_path, ignore_case})
    }
}
```

```rust
let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
```
```
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let weight: f32 = input.trim().parse().unwrap();
```


```rust
pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;
    //use the carriage return to return the cursor to the beginning of the line so that
    //it will overwrite the previous value
    if !silent {
        eprint!("\rtotal bytes: {}", total_bytes);
        if last {
            eprintln!();
        }
    }
}

//main()
let mut total_bytes = 0;
loop {
    let buf = match read::read(&args.infile) {
        Ok(x) if x.is_empty() => break,
        Ok(x) => x,
        Err(_) => break,
    };
    stats::stats(args.silent, buf.len(), &mut total_bytes, false);
    //false means stop the program cleanly
    // true means keep going
    if !write::write(&args.outfile, &buf)? {
        break;
    }
}    
stats::stats(args.silent, 0, &mut total_bytes, true);
```


```rust
//main()
let quit = Arc::new(Mutex::new(false));
let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());
let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit2));
let write_handle = thread::spawn(move || write::write_loop(&outfile, quit3));


pub fn read_loop(infile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    //... send this buffer to the stats thread
    let mut quit = quit.lock().unwrap();
    *quit = true;
    Ok(())
}
```

```rust
let a = 0x12C;
println!("base 16: {:x}", a);

let a: u16 = 100;
let b: i32 = a.try_into().unwrap();

let a: f32 = 0.3;
println!("{}", a.to_bits());

let res: f32 = 0.1 + 0.1;
let desired: f32 = 0.2;
assert!((desired - res).abs() <= f32::EPSILON>);

let x: f32 = 1.0/0.0;
assert!(!x.is_finite());
```


- for item in container <=>  for item in IntoIterator::into_iter(container)  <=> Ownership

- for item in &container {} <=> for item in container.iter()  <=> Read-only

- for item in &mut container {} <=> for item in container.iter_mut()  <=> Read-write

```rust
use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0); //one second
    let start = Instant::now();
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}
```

```rust
'outer for x in 0.. {
    for y in 0.. {
        for z in 0.. {
            if x + y + z > 1000 {
                break 'outer;
            }
        }
        // ...
    }
} 
```

Major = Breaking Change
Minor = Add Functionality
Patch = Bug Fixes

```rust
use std::convert::TryFrom;

enum NonZeroError {
    IsZero,
}
struct NonZero(i32);

//Fallible type conversion
impl TryFrom<i32> for NonZero {
    type Error = NonZeroError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(NonZeroError::IsZero)
        } else {
            Ok(NonZero(value))
        }
    }
}
```

