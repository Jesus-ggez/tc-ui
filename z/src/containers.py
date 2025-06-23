from tc_ui import HtmlElement, StyleComponent


#<·
class Container(HtmlElement):
    def __init__(self, items: list[HtmlElement]) -> None:
        super().__init__()
        self.tag = 'div'

        self.content= items
        self.set_attr_class('foo')
        self.style(
            value=StyleComponent()
                .background_color('#121212')
                .padding('4px')
                .as_inline()
        )




