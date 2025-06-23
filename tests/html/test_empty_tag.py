import pytest


#¿?
from tc_ui import HtmlElement


#<·
# --- Test default value ---
def create_default_tag() -> HtmlElement:
    return HtmlElement()


class HtmlElemetSubClass(HtmlElement):
    def __init__(self) -> None:
        super().__init__()


class HtmlElemetSubClassWithParams(HtmlElement):
    def __init__(self, param: str = '', tag: str = 'div') -> None:
        super().__init__()


_div_: str = '<div></div>'
@pytest.mark.parametrize('element_to_test,expected_output', [
    (create_default_tag, _div_),
    (lambda: HtmlElemetSubClass(), _div_),
    (lambda: HtmlElemetSubClassWithParams(), _div_),
    (lambda: HtmlElemetSubClassWithParams(param='any'), _div_),
    (lambda: HtmlElemetSubClassWithParams(param='any', tag='div'), _div_),
    (lambda: HtmlElemetSubClassWithParams(tag='div', param='any'), _div_),
    (lambda: HtmlElemetSubClassWithParams(tag='div'), _div_),
    (lambda: HtmlElement(), _div_),
])
def test_default_item(element_to_test, expected_output) -> None:
    item = element_to_test()

    assert str(item) == expected_output
    assert item.__str__() == expected_output
    assert item.as_tag() == expected_output
    assert item.formated() == expected_output


# --- Test renamed value ---
def create_renamed_tag() -> HtmlElement:
    return HtmlElement('renamed')


class RenamedHtmlElemetSubClass(HtmlElement):
    def __init__(self) -> None:
        super().__init__()
        self.tag = 'renamed'


class RenamedHtmlElemetSubClassWithParams(HtmlElement):
    def __init__(self, param: str = '', tag: str = 'renamed') -> None:
        super().__init__()
        self.tag = tag


_renamed_: str = '<renamed></renamed>'
@pytest.mark.parametrize('element_to_test,expected_output', [
    (create_renamed_tag, _renamed_),
    (lambda: RenamedHtmlElemetSubClass(), _renamed_),
    (lambda: RenamedHtmlElemetSubClassWithParams(), _renamed_),
    (lambda: RenamedHtmlElemetSubClassWithParams(param='any'), _renamed_),
    (lambda: RenamedHtmlElemetSubClassWithParams(param='any', tag='renamed'), _renamed_),
    (lambda: RenamedHtmlElemetSubClassWithParams(tag='renamed', param='any'), _renamed_),
    (lambda: RenamedHtmlElemetSubClassWithParams(tag='renamed'), _renamed_),
    (lambda: HtmlElement('renamed'), _renamed_),
])
def test_renamed_item(element_to_test, expected_output) -> None:
    item = element_to_test()

    assert str(item) == expected_output
    assert item.__str__() == expected_output
    assert item.as_tag() == expected_output
    assert item.formated() == expected_output

# --- Test not named value ---
def create_not_named_tag() -> HtmlElement:
    return HtmlElement('')


class NotnamedHtmlElemetSubClass(HtmlElement):
    def __init__(self) -> None:
        super().__init__()
        self.tag = ''


class NotnamedHtmlElemetSubClassWithParams(HtmlElement):
    def __init__(self, param: str = '', tag: str = '') -> None:
        super().__init__()
        self.tag = tag


_not_named_: str = '<></>'
@pytest.mark.parametrize('element_to_test,expected_output', [
    (create_not_named_tag, _not_named_),
    (lambda: NotnamedHtmlElemetSubClass(), _not_named_),
    (lambda: NotnamedHtmlElemetSubClassWithParams(), _not_named_),
    (lambda: NotnamedHtmlElemetSubClassWithParams(param='any'), _not_named_),
    (lambda: NotnamedHtmlElemetSubClassWithParams(param='any', tag=''), _not_named_),
    (lambda: NotnamedHtmlElemetSubClassWithParams(tag='', param='any'), _not_named_),
    (lambda: NotnamedHtmlElemetSubClassWithParams(tag=''), _not_named_),
    (lambda: HtmlElement(''), _not_named_),
])
def test_not_named_item(element_to_test, expected_output) -> None:
    item = element_to_test()

    assert str(item) == expected_output
    assert item.__str__() == expected_output
    assert item.as_tag() == expected_output
    assert item.formated() == expected_output

