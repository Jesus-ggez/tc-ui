from tc_ui import StyleComponent, HtmlElement


def test_HtmlElement() -> None:
    test_str_element()
    test_as_tag_element()
    test_as_tag_with_elements()


def test_str_element() -> None:
    print(
        '__str__\n', test_element()
    )

def test_as_tag_element() -> None:
    print(
        '__tag__\n', test_element().as_tag()
    )

def test_as_tag_with_elements() -> None:
    child: HtmlElement = HtmlElement()
    child.append(child.as_tag())

    tag: HtmlElement = test_element()
    tag.append(child.as_tag())
    tag.append(tag.as_tag())
    tag.append(tag.as_tag())
    tag.append(tag.as_tag())

    print('__components__\n', tag.as_tag())

def test_element() -> HtmlElement:
    return (
        HtmlElement()
        .set_attr('set', 'attr')
        .set_class('cls')
    )

def test_component() -> StyleComponent:
    return (
        StyleComponent()
        .border_radius('4px')
        .padding('10px')
        .margin('1px')
    )


def test_as_tag() -> None:
    tag: str = test_component().as_tag('.test-tag')

    print(tag)


def test_as_class() -> None:
    clss: str = test_component().as_class('.test-cls')

    print(clss)


def test_as_inline(b: bool) -> None:
    inline: str = test_component().as_inline(b)
    print(inline)


def test_StyleComponent() -> None:
    test_as_tag()
    test_as_class()
    test_as_inline(True)
    test_as_inline(False)


def main() -> None:
    test_StyleComponent()
    print(); print( )

    test_HtmlElement()


if __name__ == '__main__':
    main()
