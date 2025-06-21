from tc_ui import StyleComponent


#<·
class Palette(StyleComponent):
    def __init__(self) -> None:
        super().__init__(
            {
                'padding': '10px'
            }
        )
        self.background_color('cyan')
