
extern crate std;
use ramhorns_ext::Template;
use ramhorns_derive::Content;
fn main() {
    let v1 = Sa {
        prop1: vec![Value::new(1), Value::new(2)],
        prop2: 12,
        #[cfg(feature = "chrono")]
        create_time: chrono::Utc::now().naive_utc(),
        #[cfg(feature = "uuid")]
        id: uuid::Uuid::parse_str("02f09a3f-1624-3b1d-8409-44eff7708208").unwrap(),
    };
    let s = "
            {{?prop1}}
                {{#prop1}}
                   {{value}} , id: {{id}}, {{prop2}} , create_time: {{create_time}}
                {{/prop1}}
            {{/prop1}}
    ";
    
    let tpl = Template::new(s).unwrap();
    let rst = tpl.render(&v1);
    println!("rst = {}", rst);
}

#[derive(Debug, Content)]
struct Sa {
    prop1: Vec<Value<u8>>,
    prop2: i32,
    #[cfg(feature = "chrono")]
    create_time: chrono::NaiveDateTime,
    #[cfg(feature = "uuid")]
    id: uuid::Uuid,
}

#[allow(unused)]
#[derive(Content, Debug)]
pub struct Value<T> {
    pub value: T
}

impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }
}

// impl ::ramhorns::Content for Sa {
//     #[inline]
//     fn capacity_hint(&self, tpl: &::ramhorns::Template) -> usize {
//         tpl.capacity_hint() + self.prop1.capacity_hint(tpl)
//             + self.prop2.capacity_hint(tpl)
//     }
//     #[inline]
//     fn render_section<C, E>(
//         &self,
//         section: ::ramhorns::Section<C>,
//         encoder: &mut E,
//     ) -> std::result::Result<(), E::Error>
//     where
//         C: ::ramhorns::traits::ContentSequence,
//         E: ::ramhorns::encoding::Encoder,
//     {
//         section.with(self).render(encoder)
//     }
//     #[inline]
//     fn render_field_escaped<E>(
//         &self,
//         hash: u64,
//         name: &str,
//         encoder: &mut E,
//     ) -> std::result::Result<bool, E::Error>
//     where
//         E: ::ramhorns::encoding::Encoder,
//     {
//         match hash {
//             18071763218198546876u64 => self.prop1.render_escaped(encoder).map(|_| true),
//             18072889118105645715u64 => self.prop2.render_escaped(encoder).map(|_| true),
//             _ => Ok(false),
//         }
//     }
//     #[inline]
//     fn render_field_unescaped<E>(
//         &self,
//         hash: u64,
//         name: &str,
//         encoder: &mut E,
//     ) -> std::result::Result<bool, E::Error>
//     where
//         E: ::ramhorns::encoding::Encoder,
//     {
//         match hash {
//             18071763218198546876u64 => self.prop1.render_unescaped(encoder).map(|_| true),
//             18072889118105645715u64 => self.prop2.render_unescaped(encoder).map(|_| true),
//             _ => Ok(false),
//         }
//     }
//     fn render_field_section<P, E>(
//         &self,
//         hash: u64,
//         name: &str,
//         section: ::ramhorns::Section<P>,
//         encoder: &mut E,
//     ) -> std::result::Result<bool, E::Error>
//     where
//         P: ::ramhorns::traits::ContentSequence,
//         E: ::ramhorns::encoding::Encoder,
//     {
//         match hash {
//             18071763218198546876u64 => {
//                 self.prop1.render_section(section, encoder).map(|_| true)
//             }
//             18072889118105645715u64 => {
//                 self.prop2.render_section(section, encoder).map(|_| true)
//             }
//             _ => Ok(false),
//         }
//     }
//     fn render_field_inverse<P, E>(
//         &self,
//         hash: u64,
//         name: &str,
//         section: ::ramhorns::Section<P>,
//         encoder: &mut E,
//     ) -> std::result::Result<bool, E::Error>
//     where
//         P: ::ramhorns::traits::ContentSequence,
//         E: ::ramhorns::encoding::Encoder,
//     {
//         match hash {
//             18071763218198546876u64 => {
//                 self.prop1.render_inverse(section, encoder).map(|_| true)
//             }
//             18072889118105645715u64 => {
//                 self.prop2.render_inverse(section, encoder).map(|_| true)
//             }
//             _ => Ok(false),
//         }
//     }
//     fn render_field_notnone_section<P, E>(
//         &self,
//         hash: u64,
//         name: &str,
//         section: ::ramhorns::Section<P>,
//         encoder: &mut E,
//     ) -> std::result::Result<bool, E::Error>
//     where
//         P: ::ramhorns::traits::ContentSequence,
//         E: ::ramhorns::encoding::Encoder,
//     {
//         match hash {
//             18071763218198546876u64 => Ok(self.prop1.is_truthy()),
//             18072889118105645715u64 => Ok(self.prop2.is_truthy()),
//             _ => Ok(false),
//         }
//     }
// }