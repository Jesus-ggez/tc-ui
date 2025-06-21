from tc_ui import HtmlElement, StyleComponent


#.?
from .texts import Title, Paragraph


#<·
class Index(HtmlElement):
    def __init__(self) -> None:
        super().__init__()
        self.tag = 'html'

        self.components = [
            self.head().formated(),
            self.body().formated(),
        ]

    def __red_box(self) ->  StyleComponent:
        return(
            StyleComponent()
            .padding('10px')
            .background_color('#dc143c')
            .margin('1px')
        )


    def head(self) -> HtmlElement:
        head: HtmlElement = HtmlElement(tag='head')
        head.components = self.__headers()

        return head


    def body(self) -> HtmlElement:
        body: HtmlElement = HtmlElement(tag='body')
        body.components = self.__controls()
        body.append(Paragraph().formated())
        body.append(self.__red_box().as_tag('.red-box'))

        return body


    def __headers(self) -> list:
        data: list = []

        base_tag: HtmlElement = HtmlElement('meta')
        for i in range(10):
            base_tag.components = [f'foo {i}']

            data.append(base_tag.as_tag())

        return data


    def __controls(self) -> list:
        return [
            Title().as_tag()
            for _ in range(10)
        ]
