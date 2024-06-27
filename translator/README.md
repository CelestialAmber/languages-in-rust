## Running
You first need to create a .env file in the toplevel directory (`languages-in-rust/.env`)
and put in it `ANTHROPIC_API_KEY=<your anthropic key>`. If you don't know what that is,
see the [anthropic page](https://www.anthropic.com/api) and yes you have to pay per use
for the api (but it's cents unless you want to feed whole books here).

`cat <the text you want to translate> | cargo run` while being in `translator` will
print out a translation of your text in Rust. If you want to type freely, do `cargo run`,
type what you want, and then hit enter and ctrl-D to translate.

Unfortunately [this crate](https://github.com/AbdelStark/anthropic-rs/tree/main) I found
for the anthropic API only seems to support an ancient API as of time of writing, which
means this is stuck with an ancient and bad Claude model until someone improves that
crate. If you feel like doing that, go
[here](https://github.com/AbdelStark/anthropic-rs/blob/main/anthropic/src/client.rs)
and see if you can make an alternative to `complete()` that follows
[this API](https://docs.anthropic.com/en/api/messages). The output still looks ok to me,
though if you want to translate entire books it's probably worth the effort.

If you need to print longer bytes, change `max_tokens_to_sample` in the code to
something longer.

Output of the toplevel README.md, fed into this:
```
 fn main() {
   println!("# languages-in-rust");
   println!("Rewrite all the languages in Rust x3");

   println!("");

   println!("# FAQ");

   println!("## What is this?");
   println!("");
   println!("This is a project dedicated to rewriting every language in Rust. The idea is to eventually use the Rust code to ~~create a mobile learning application better than Luodingo~~ make it more fun to learn the languages by reading them in Rust while wearing programming socks :3");

   println!("");

   println!("## Will this make me native in my TL?");
   println!("");
   println!("I cannot guarantee that, but it will make you native in Rust, which has good native shocking potential.");

   println!("");

   println!("## How can I contribute?");
   println!("");
   println!("Feel free to send pull requests.");
}
```

This is a joke, well < 100 lines of code, don't sue me please.
