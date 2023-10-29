Fast [**Mustache**](https://mustache.github.io/) template engine implementation
in pure Rust.

**Ramhorns-ext** is base on **Ramhorns**, it loads and processes templates **at runtime**. It comes with a derive macro
which allows for templates to be rendered from native Rust data structures without doing
temporary allocations, intermediate `HashMap`s or what have you.

With a touch of magic 🎩, the power of friendship 🥂, and a sparkle of
[FNV hashing](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function)
✨, render times easily compete with static template engines like
[**Askama**](https://github.com/djc/askama).

What else do you want, a sticker?

### Cargo.toml

```toml
[dependencies]
ramhorns-ext = { version = "0.17", features = ["chrono", "uuid"] }
```

### Example

```rust
use ramhorns_ext::{Template, Content};

#[derive(Content)]
struct Post<'a> {
    id: uuid::Uuid,
    create_time: chrono::NaiveDateTime,
    title: &'a str,
    teaser: &'a str,
}

#[derive(Content)]
struct Blog<'a> {
    title: String,        // Strings are cool
    posts: Vec<Post<'a>>, // &'a [Post<'a>] would work too
}

// Standard Mustache action here
let source = "<h1>{{title}}</h1>\
              {{#posts}}<article><h2>{{id}}</h2><h2>{{title}}</h2><p>{{teaser}}</p></article>{{/posts}}\
              {{^posts}}<p>No posts yet :(</p>{{/posts}}";

let tpl = Template::new(source).unwrap();

let rendered = tpl.render(&Blog {
    title: "My Awesome Blog!".to_string(),
    posts: vec![
        Post {
            id: uuid::Uuid::parse_str("02f09a3f-1624-3b1d-8409-44eff7708201").unwrap(),
            create_time: chrono::Utc::now().naive_utc(),
            title: "How I tried Ramhorns and found love 💖",
            teaser: "This can happen to you too",
        },
        Post {
            id: uuid::Uuid::parse_str("12f09a3f-1624-3b1d-8409-44eff7708202").unwrap(),
            create_time: chrono::Utc::now().naive_utc(),
            title: "Rust is kinda awesome",
            teaser: "Yes, even the borrow checker! 🦀",
        },
    ]
});

assert_eq!(rendered, "<h1>My Awesome Blog!</h1>\
                      <article>\
                          <h2>02f09a3f-1624-3b1d-8409-44eff7708201</h2>\
                          <h2>How I tried Ramhorns and found love 💖</h2>\
                          <p>This can happen to you too</p>\
                      </article>\
                      <article>\
                          <h2>12f09a3f-1624-3b1d-8409-44eff7708202</h2>\
                          <h2>Rust is kinda awesome</h2>\
                          <p>Yes, even the borrow checker! 🦀</p>\
                      </article>");
```

### Features

+ {{$value}} for Vec<T> etc.
+ uuid and chrono DateTime support
+ Rendering sections `{{?foo}} ... {{/foo}}`.
+ Rendering common types, such as `&str`, `String`, `bool`s, and numbers into `{{variables}}`.
+ Unescaped printing with `{{{tripple-brace}}}` or `{{&ampersant}}`.
+ Rendering sections `{{#foo}} ... {{/foo}}`.
+ Rendering inverse sections `{{^foo}} ... {{/foo}}`.
+ Rendering partials `{{>file.html}}`.
+ Zero-copy [CommonMark](https://commonmark.org/) rendering from fields marked with `#[md]`.

### Benches

Rendering a tiny template:
```
test a_simple_ramhorns            ... bench:          82 ns/iter (+/- 4) = 1182 MB/s
test b_simple_askama              ... bench:         178 ns/iter (+/- 8) = 544 MB/s
test c_simple_tera                ... bench:         416 ns/iter (+/- 98) = 233 MB/s
test c_simple_tera_from_serialize ... bench:         616 ns/iter (+/- 33) = 157 MB/s
test d_simple_mustache            ... bench:         613 ns/iter (+/- 34) = 158 MB/s
test e_simple_handlebars          ... bench:         847 ns/iter (+/- 40) = 114 MB/s
```

Rendering a tiny template with partials:
```
test pa_partials_ramhorns         ... bench:          85 ns/iter (+/- 7) = 1141 MB/s
test pb_partials_askama           ... bench:         210 ns/iter (+/- 9) = 461 MB/s
test pc_partials_mustache         ... bench:         827 ns/iter (+/- 39) = 117 MB/s
test pd_partials_handlebars       ... bench:         846 ns/iter (+/- 29) = 114 MB/s
```

Compiling a template from a string:
```
test xa_parse_ramhorns            ... bench:         190 ns/iter (+/- 10) = 821 MB/s
test xb_parse_mustache            ... bench:       3,229 ns/iter (+/- 159) = 48 MB/s
test xe_parse_handlebars          ... bench:       6,883 ns/iter (+/- 383) = 22 MB/s
```

Worth noting here is that [**Askama**](https://github.com/djc/askama) is processing
templates at compile time and generates static rust code for rendering. This is great
for performance, but it also means you can't swap out templates without recompiling
your Rust binaries. In some cases, like for a static site generator, this is
unfortunately a deal breaker.

Parsing the templates on runtime is never going to be free, however **Ramhorns** has
a really fast parser built on top of [**Logos**](https://github.com/maciejhirsz/logos),
that makes even that part of the process snappy.

The [**Mustache** crate](https://github.com/nickel-org/rust-mustache) is the closest
thing to **Ramhorns** in design and feature set.

### License

Ramhorns is free software, and is released under the terms of the GNU General Public
License version 3. See [LICENSE](LICENSE).
