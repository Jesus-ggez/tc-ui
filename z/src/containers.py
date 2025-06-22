from tc_ui import HtmlElement, StyleComponent


#<·
class Container(HtmlElement):
    def __init__(self, items: list[HtmlElement]) -> None:
        super().__init__()
        self.tag = 'div'

        self.components = [ i.__str__() for i in items ]
        self.set_class('foo')
        self.style(
            value=StyleComponent()
                .background_color('#121212')
                .padding('4px')
                .as_inline()
        )




