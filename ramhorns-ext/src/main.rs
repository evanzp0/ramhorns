
use ramhorns_derive::Content;
use ramhorns_ext::Template;
fn main() {
    let v1 = Sa {
        bb: Sb {
            name: "my_name".to_owned(),
            prop1: vec![1, 2],
        },
        opt1: Some(vec![11, 22]),
    };

    let s = "
        {{?bb prop1}}
            {{#bb prop1}}
                {{$value}} a,
            {{/bb prop1}}
        {{/bb prop1}}
        {{?opt1}}
            {{#opt1}}
                {{$value}}
            {{/opt1}}
        {{/opt1}}
    ";

    // let s = "
    //     {{#bb}}
    //         {{?prop1}}
    //             {{$value}} a,
    //         {{/prop1}}
    //     {{/bb}}
    // ";

    // let s = "
    //     {{#bb}}
    //         {{?prop1}}
    //             {{#prop1}}
    //                 {{$value}} a,
    //             {{/prop1}}
    //         {{/prop1}}
    //     {{/bb}}
    // ";

    // let s = "
    //     {{?bb prop1}}
    //         {{#bb prop1}}
    //             {{$value}} a,
    //         {{/bb prop1}}
    //     {{/bb prop1}}
    // ";

    let tpl = Template::new(s).unwrap();
    let rst = tpl.render(&v1);
    println!("rst = {}", rst)
}

#[derive(Content)]
struct Sa {
    bb: Sb,
    opt1: Option<Vec<i32>>,
}

#[derive(Content)]
struct Sb {
    name: String,
    prop1: Vec<u8>,
}
