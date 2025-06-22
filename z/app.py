from src.components import Index


def main() -> None:
    try:
        a = Index()

        try:
            b = a.as_tag()
            print('a.formated() -> ', b)

        except Exception as e:
            ...
    except Exception as e:
        ...


if __name__ == '__main__':
    main()
