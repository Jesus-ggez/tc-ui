from tc_ui import HtmlElement, StyleComponent, Text


class Card(HtmlElement):
    def __init__(self) -> None:
        super().__init__()

        self.tag = 'section'

        self.set_attr_class(
            'card'
        )
        self.extend_html(
            [
                self.__style().as_tag('.card'),
                StyleComponent()
                .background_color('gray')
                .color('cyan')
                .as_tag('.blue')
            ]
        )

        self.append(
            self.__title()
            .set_attr_class('blue')
        )


    def __style(self) -> StyleComponent:
        return (
            StyleComponent()
            .background_color('#cd134c')
            .border_radius('4px')
            .padding('10px')
            .height('10%')
            .width('10rem')
            .margin('1px')
        )


    def __title(self) -> Text:
        return Text(
            value='My Card',
            tag='h1',
        )
