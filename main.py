from tc_ui import HtmlElement, StyleComponent

class MyComponent(HtmlElement):
    def __init__(self) -> None:
        super().__init__()

        self.style(value=self.Sitem().inline(False))
        self.set_class('red-box')
        self.href('docs-tring')

        self.components = [
            HtmlElement()
            .set_class('red-box')
            .style(value=self.Sitem().inline(False))
            .as_tag()
            for _ in range(10)
        ]

    def Sitem(self) -> StyleComponent:
        return (
            StyleComponent()
            .background_color('#dc143c')
            .border_radius('4px')
            .padding('10px')
        )


if __name__ == '__main__':
    zz = (
        StyleComponent()
        .padding('10px')
        .margin('1px')
    )
    z = HtmlElement('style')
    z.append(zz.as_class('dot'))
    z.append(zz.as_class('env'))

    print(z.formated())
    print(zz)
