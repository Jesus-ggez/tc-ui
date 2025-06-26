def replace_types(s: str) -> str:
    return (
        s
        .replace('kwargs:Option<dict>', '**kwargs')
        .replace('Option', 'Optional')
        .replace('HashMap', 'dict')
        .replace('String', 'str')
        .replace('Vec', 'list')
    )


def replace_no_pytypes(s: str) -> str:
    return (
        s
        .replace('PyString', 'str')
        .replace('PyList', 'list')
        .replace('PyDict', 'dict')
        .replace('PyResult', '')
        .replace('PyRefMut', '')
        .replace('PyRef', '')
        .replace('Self', 'self,')
        .replace('Bound', '')
        .replace('mut', '')
        .replace('slf:', '')
    )


def replace_skw(s: str) -> str:
    return (
        s
        .replace('<', '[')
        .replace('>', ']')
        .replace(',', ', ')
        .replace(':', ': ')
    )


def firs_alnum(s: str) -> str:
    new_s: str = ''

    for char in s:
        if char.isalnum() or char == '_':
            new_s += char
            continue

        if new_s:
            break

    return new_s


def remove_invalid_chars(string: str) -> str:
    string = replace_no_pytypes(string).strip()
    valid_chars: str = '<:,_'
    data: list[str] = []
    skip: bool = False
    skw: int = 0

    if not string:
        return ''

    for s in string:
        if s.isalnum():
            skip = False
            data.append(s); continue

        if not s in valid_chars + '>':
            continue

        if (
            not ('' if not data else data[-2]).isalnum() and
            not ('' if not data else data[-1]).isalnum() and
            not s.isalnum()
        ): continue

        if skip: continue

        # complete
        if s == '<':
            if data and not data[-1].isalnum():
                continue

            skw += 1
            skip = True

        # complete
        if s == '>':
            s = '>' * skw
            s += ','
            skw = 0

        if s == ':':
            skip = True

        if s == data[-1]:
            continue

        data.append(s)

    res: str = (
        ''.join(data)
        .replace(',,', ',')
        .strip(',')
    )
    res = replace_types(res)
    res = replace_skw(res)

    #if res: print(res)
    return res


















