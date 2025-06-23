from tc_ui import (
    StyleComponent,
    HtmlElement,
)


class TestHtml:
    def __init__(self) -> None:
        self.__build()


    def __build(self) -> None:
        self._empty()
        self._has_content()


    def _empty(self) -> None:
        item: HtmlElement = HtmlElement()
        print('empty -> ', item)


    def _has_content(self) -> None:
        item: HtmlElement = HtmlElement(
            tag='content',
        )
        print('has content -> ', item)


class TestStyle:
    def __init__(self) -> None:
        self.__build()


    def __build(self) -> None:
        self._empty()
        self._has_content()



    def _has_content(self) -> None:
        item: StyleComponent = StyleComponent(
            properties={
                'has': 'content'
            }
        )
        print('has content -> ',item.as_inline())


def main() -> None:
    TestHtml()
    TestStyle()

if __name__ == '__main__':
    main()
