template: str = """use pyo3::prelude::*;

use crate::static_component::StaticComponent;

#[pymethods]
impl StaticComponent{
    #[new]
    fn new() -> Self {
        Foo {
            properties: HashMap::new(),
        }
    }
    fn set_property(mut slf: PyRefMut<'_, Self>, name: String, value: String) -> PyRefMut<'_, Self> {
        let _ = slf.properties.insert(name, value);
        slf
    }

    __methods__
}"""

def fn_builder(prop: str) -> str:
    quote_2: str = '"'

    data: str = """
    // set __prop__u
    fn set___prop__(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, .?__prop__u.?.to_string(), value)
    }

    """

    return ( data
        .replace('__prop__u', prop)
        .replace('__prop__', prop.replace('-', '_'))
        .replace('.?', quote_2)
    )


def main() -> None:
    with open('zzz.txt', 'r') as data:
        raw: str = ''

        for line in data:
            begin: str = line.split(':')[0].strip()

            raw += fn_builder(begin)


        with open('css_properties.rs', 'w') as d:
            d.write(
                template.replace('__methods__', raw)
            )

if __name__ == '__main__':
    main()
