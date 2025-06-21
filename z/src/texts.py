from tc_ui import Text


#.?
from .styled import Palette


#<·
class Title(Text):
    def __init__(self) -> None:
        super().__init__(
            #value='Hello world!',
        )
        self.value = 'Hello world!'

        self.set_class('red-box')


class Paragraph(Text):
    def __init__(self) -> None:
        super().__init__()
        self.value = 'Palette'

        self.style(Palette().as_inline(False))
