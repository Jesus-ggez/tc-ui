from generators.stub_gen import app

#<·
def document(name: str) -> tuple[str]:
    data: list = []
    with open(name, 'r') as raw:
        for line in raw:
            data.append(line.strip())

    return tuple(data)


def add_data(value: str) -> None:
    target: str = 'html_properties.rs'
    with open(target, 'r') as doc:
        new_data: str = doc.read().replace('//__data__', value)

        with open('z' + target, 'w') as new_doc:
            new_doc.write(new_data)


def template_fn(name: str) -> str:
    data: list[str] = [
        "fn set_.?.(slg: PyRefMut<'_, Self>, value: String)"
            .replace('.?.', name).replace('-', '_', 1)
            + " -> PyRefMut<'_, Self> {",
        f'Self::set_attr(slf, "{name}".to_string(), value)',
        '}'
    ]

    return '\n'.join(data)


def main() -> None:
    new_document: list[str] = []
    divider: str = ' - '

    for line in document(name='data.txt'):
        if not divider in line:
            continue

        new_line: str = line.split(divider)[0].strip()
        new_document.append(
            template_fn(name=new_line),
        )

    add_data(''.join(new_document))



if __name__ == '__main__':
    main()
