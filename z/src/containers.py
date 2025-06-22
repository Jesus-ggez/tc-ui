from tc_ui import HtmlElement, StyleComponent


#<·
class Container(HtmlElement):
    def __init__(self, items: list[HtmlElement] | None = None) -> None:
        super().__init__()
        self.tag = 'div'

        if items is None:
            return

        self.components = [ i.as_tag() for i in items ]
        self.set_class('foo')
        self.style(
            value=StyleComponent()
                .background_color('#121212')
                .padding('4px')
                .as_inline()
        )


def safe_exec(func):
    print(func)
    def wrapper(*args, **kwargs):
        try:
            result = func(*args, **kwargs)
            return result

        except Exception as e:
            print(e)
    return wrapper

