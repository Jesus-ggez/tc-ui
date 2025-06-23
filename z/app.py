from src.components import Index


def main() -> None:
    a = Index()

    b = a.as_tag()
    print('a.formated() -> ', b)

    with open('xample.txt', 'w') as foo:
        foo.write(b.__str__())



if __name__ == '__main__':
    main()
