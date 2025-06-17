from tc_ui import (
    StyleComponent,
    HtmlElement,
    Colors,
)


#<·
class UserCard(HtmlElement):
    def __init__(self, data: dict) -> None:
        super().__init__(
            style=StyleComponent(),
            id='user-card'
        )
        self.data: dict = data

        self.style\
            .set_background_color('#dc143c')\
            .set_border_radius('4px')\
            .set_padding('10px')\
            .set_border_color('red')

        self.content(
            SecondaryText('Status'),
            Container( # equivalencia a un div puro
                # default { order='col', ... }
                items=[
                    Container(
                        order='col', # arriba a abajo
                        items=[
                            AvatarBall(),
                            MainText(),
                        ],
                    ),
                    Container(
                        order='row', # izq a der
                        items=[
                            FilledButton('User Info'),
                            FilledButton('Configuration'),
                            FilledButton('Tools'),
                        ],
                    ),
                    Container(
                        grid_struct=(5, 5), # tuple[col, row]
                        grid=True,
                        items=[
                            self.image_component(auto_id=iden)
                            for iden in self.data.value
                        ],
                    )
                ]
            ),
        )


    def image_component(self, auto_id: str) -> HtmlElement:
        return HtmlElement(
            style=StyleComponent()\
                .set_border_radius('2')\
                .set_padding('2px')\
                .set_border_color(Colors.TRANSPARENT),
            id=auto_id,
        )
