use tera::Tera;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

fn print_of_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn take_template(name: &str) -> String {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render(name, &context).unwrap();
    page_content
}
