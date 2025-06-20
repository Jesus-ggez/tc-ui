t: str = '    '


class Macros:
    py_cls: str = '#[pymethods]'


class Magic:
    impl: str = 'impl'
    py_methods: str = 'fn '


def struc_class(name: str) -> list[str]:
    return [
        f'class {name}:',
        t + 'def __init__(self) -> None:',
        t + t + '...\n\n',
    ]


def tokenize(word: str) -> list[str]:
    data: list[str] = []
    valids: tuple = ('_', )
    new_word: str = word[0]

    for char in word[1:]:
        c_char_type: bool = new_word[-1].isalnum() or new_word[-1] in valids

        if c_char_type != char.isalnum() and not (char in valids):
            data.append(new_word)
            new_word = ''

        new_word += char

    data.append(new_word)
    return data


def stub_gen(doc: tuple[str]) -> dict:
    classes: dict = {}
    slf: str = "slf: PyRefMut<'_, Self>,"
    t: str = '    '

    template_class_methods: str = t + "def .?.(self, ~>) -> __cls__: ...\n"
    current_class: str = ''

    for line in doc:
        if not line.startswith(Macros.py_cls):
            continue

        if line.endswith('#.?'):
            continue

        Macros.py_cls = ''

        if 'new' in line:
            continue

        if line.startswith(Magic.impl):
            class_name: str = line\
                .removeprefix(Magic.impl)\
                .strip()\
                .removesuffix('{')\
                .strip()

            current_class = class_name

            classes[class_name] = struc_class(name=class_name)
            continue

        if not current_class:
            continue

        if not line.startswith(Magic.py_methods):
            continue

        fn_line: str = line\
            .removeprefix(Magic.py_methods)\
            .split('->')[0]\
            .strip()\
            .removesuffix(')')\
            .strip()\
            .replace(slf, '')\
            .replace('mut', '')

        tokens: list[str] = tokenize(fn_line)

        s_current_class = current_class
        if line.endswith('!'):
            s_current_class = line.split('<·')[1]\
                .split('·>')[0]

        func: str = template_class_methods\
            .replace( '~>',''.join(tokens[2:]))\
            .replace(
                '__cls__',
                s_current_class if s_current_class != current_class else repr(s_current_class)
            )\
            .replace('.?.', tokens[0])\

        classes[current_class].append(
            replace_to_pytypes(func)
            .replace('self, self, ', 'self, ')
            .replace('self, self', 'self')
            .replace('self, )', 'self)')
        )

    return classes


def replace_to_pytypes(word) -> str:
    nums: tuple = ('i32', 'i16', 'i8', 'isize', 'i64', 'u32', 'u16', 'u8', 'usize', 'u64')
    new_word: str = word\
        .replace('String', 'str')

    for num in nums:
        new_word = new_word\
            .replace(num, 'int')


    return new_word


def app(doc: tuple[str]) -> None:
    data: dict = stub_gen(doc=doc)

    for k in data.keys():
        try:
            with open('zample.py', 'a') as document:
                document.write('\n'.join(data[k]))
                document.write('\n\n')

        except:
            with open('zample.py', 'w') as document:
                document.write('\n'.join(data[k]))
                document.write('\n\n')
