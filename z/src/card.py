from tc_ui import HtmlElement, StyleComponent, Text


class Card(HtmlElement):
    def __init__(self) -> None:
        super().__init__()

        self.tag = 'section'

        self.set_attr_class(
            'card'
        )
        self.append_html(
            self.__style().as_tag('.card')
        )

        self.append(
            self.__title()
        )


    def __style(self) -> StyleComponent:
        return (
            StyleComponent()
            .background_color('#cd134c')
            .padding('10px')
            .margin('1px')
            .border_radius('1px')
            .width('10vh')
            .height('10vh')
        )


    def __title(self) -> Text:
        return Text(
            value='My Card',
            tag='h1',
        )
