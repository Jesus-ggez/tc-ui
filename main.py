import os


#<·
def read_as_list(filename: str) -> list[str]:
    with open(filename, 'r') as data:
        return [ l.strip() for l in data.readlines() ]


def get_first_alnum_word(s: str) -> str:
    new_word: str = ''

    for char in s:
        if char.isalnum() or char == '_':
            new_word += char
            continue

        if new_word:
            break

    return new_word




class Pyo:
    data: dict = {}


class PyoFile:
    def __init__(self, item: str) -> None:
        self._cls_name: str = ''
        self._params: list = []
        self._item: str = item

        self.__build()


    def __build(self) -> None:
        try:
            self.__remove_trash()
            self.__create_content()

            if not self._cls_name in Pyo.data:
                return

            self.__format_content()

        except Exception as e:
            print(e, self._item)


    def __remove_trash(self) -> None:
        self._pure_content: list[str] = []
        counter: int = 0

        file_: list[str] = read_as_list(self._item)

        for line in file_:
            if not '#[pymethods]' in line:
                counter += 1
                continue

            self._pure_content.extend(file_[counter:])
            return


    def __create_content(self) -> None:
        is_assign_classname: bool = False
        is_fn: bool = False

        params: list = []

        for line in self._pure_content:
            if not is_assign_classname and line.startswith('impl'):
                raw: str = line.removeprefix('impl')

                self._cls_name += self.___get_alnum_str(v=raw)
                is_assign_classname = True
                continue

            if line.startswith('fn') or line.startswith('pub'):
                is_fn = True

            if 'new' in line or '__(' in line:
                is_fn = False

            if not is_fn:
                continue

            params.append(line)

            if '{' in line:
                is_fn = False
                self._params.append(params.copy())
                params.clear()


    def ___get_alnum_str(self, v: str) -> str:
        return get_first_alnum_word(s=v)

    def __format_content(self) -> None:
        for _item in self._params:
            base, raw_content = ''.join(_item).strip().split('(')
            mid, end = raw_content.split(')')

            mid = mid\
                .replace(', ', ',')\
                .replace(',', ', ')\
                .replace('&', '')

            fn_name: str = base.split()[-1]

            retrn_type: str = self.___get_alnum_str(end) or 'None'

            if 'Py' in retrn_type:
                new_content: str = end.replace(' ', '')

                index: int = new_content.find(retrn_type) + len(retrn_type)

                retrn_type = new_content[index:]

                retrn_type = new_content[index:][
                    retrn_type.find('<') + 1:
                    retrn_type.find('>') + 1
                ]

            if 'Self' in retrn_type:
                retrn_type = 'Self'

            if ( iz := retrn_type.count('<') ) != ( dr := retrn_type.count('>') ):
                if iz > dr:
                    retrn_type = retrn_type.replace('<', '', (iz - dr))

                if dr > iz:
                    retrn_type = retrn_type.replace('>', '', (dr - iz))


            clean_py_mid: str = ''.join(
                (
                    item + (
                        '' if item[-1].isalnum() else ' '
                    )
                    if not
                    (item[0].isalnum() and item[0] != '<')
                    or 'mut' != item
                    else ''
                )
                for item in mid.split()
                if not item.startswith('Py')
            )
            if 'Self' in clean_py_mid:
                last = clean_py_mid.find('Self')
                clean_py_mid = clean_py_mid[last:].replace('Self>', 'self')

            for item in (
                ('>', ']'), ('Self', repr(self._cls_name)),
                ('String', 'str'), ('Vec', 'list'),
                ('HashMap', 'dict'), ('<', '['),
            ):
                retrn_type = retrn_type.replace(item[0], item[1])
                clean_py_mid = clean_py_mid.replace(item[0], item[1])

            clean_mid = clean_py_mid.strip().strip(',')

            Pyo.data[self._cls_name].append(
                f'def {fn_name}({clean_mid}) -> {retrn_type}: ...'
            )



class Node:
    def __init__(self) -> None:
        self._next: str = ''

        self.__build()


    def __build(self) -> None:
        self.__create_content()


    def is_valid_node(self) -> bool:
        return 'mod.rs' in os.listdir()


    def __create_content(self) -> None:
        for item in os.listdir():
            if os.path.isdir(item):
                self._next = item
                self.___move_to()

                if self.is_valid_node():
                    self.__create_content()

                continue

            if os.path.isfile(item):
                PyoFile(item)

        os.chdir('..')


    def ___move_to(self) -> None:
        try:
            os.chdir(self._next)

        except Exception as e:
            print(e)


def create_context() -> None:
    os.chdir('src')

    for line in read_as_list('lib.rs'):
        if 'add_class' in line:
            Pyo.data[get_first_alnum_word(line[::-1])[::-1]] = []
            print(Pyo.data)


def main() -> None:
    create_context()

    Node()

    t: str = '    '
    for name, content in Pyo.data.items():
        cls_: list = [
            '',
            f'class {name}:',
            t + 'def __init__(self) -> None:',
            t + t + '...',
            '',
        ]
        cls_.extend([t + i for i in set(content)])

        cls_ = [i + '\n' for i in cls_]

        try:
            with open('foo.py', 'a') as foo:
                foo.writelines(cls_)

        except Exception as e:
            print(e)
            with open('foo.py', 'w') as foo:
                foo.writelines(cls_)


if __name__ == '__main__':
    main()
