def main() -> None:
    new_document: list[str] = []

    max_len_line: int = 0

    with open('raw_props.txt', 'r')  as raw_props:
        for line in raw_props:

            if not line.strip():
                continue

            if len(line) > max_len_line:
                max_len_line = len(line)

                print(max_len_line)

            new_document.append(
                (
                    line
                        .strip()
                        .removesuffix(';')
                        + '\n'
                )
            )

    with open('zzz.txt', 'w') as doc:
        document: list[str] = [] #or new_document
        for line in new_document:
            raw: list[str] = line.split(':')

            if len(raw) != 2:
                document.append(line)
                continue

            raw[1] = ':' + raw[1]
            diff: int = max_len_line - len(line)
            raw[0] += (' ' * diff)

            document.append(''.join(raw))

        doc.writelines(document)


def clean() -> None:
    with open('zzz.txt', 'r') as doc:
        for line in doc:
            if not (':' in line):
                print(line)

if __name__ == '__main__':
    #main()
    clean()
