# FeRuby

## What the hell is this?

After having the brilliant idea of implementing Ruby in Rust, I realized "Hey,
I've never written a programming language before. What can go wrong?!", and
created the repo. Because of the aforementioned lack of experience, however,
this will probably end up being more like a Lisp implemented in Rust, at least
for a while.

The ultimate goal is still to build a Ruby using Rust, but let's take this one
step at a time.

## Installing Rust

If you do not have Rust set up, please visit rust-lang.org for specific
instructions. In order to run this project you need Rust installed. To check
your installation simply type:

```
$ rustc --version
```

This is nowhere near stable, so expect it to be using varying versions of
Rust for the time being. I will try to keep this README updated with any
version requirements until it becomes stable-ish.

## Running the Lisp

Simply type

```
$ cargo run

```

to begin the REPL.

## Contributing

If you would like to contribute to this project (you poor thing), feel free to create a pull request. If you'd like to contact me, you can reach me at chrisccerami@gmail.com or on Twitter @chrisccerami.

- Fork it ( https://github.com/chrisccerami/fe_ruby/fork )
- Create your feature branch (git checkout -b my-new-feature)
- Commit your changes (git commit -am 'Add some feature')
- Push to the branch (git push origin my-new-feature)
- Create a new Pull Request
