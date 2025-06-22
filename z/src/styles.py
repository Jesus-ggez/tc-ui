from tc_ui import StyleComponent


#<·
class RedBoxFromDict(StyleComponent):
    def __init__(self) -> None:
        super().__init__(
            properties={
                'background-color': '#dc143c',
                'border-radius': '4px',
                'padding': '10px',
            }
        )


class BlueBoxAddDict(StyleComponent):
    def __init__(self) -> None:
        super().__init__()

        self.properties = {
            'background-color': '#efe',
            'border-radius': '4px',
            'padding': '10px',
        }


class GreenBoxPureCls(StyleComponent):
    def __init__(self) -> None:
        super().__init__()

        self.background_color('#2d572c')
        self.border_radius('4px')
        self.padding('10px')


class YellowBoxRsStyle(StyleComponent):
    def __init__(self) -> None:
        super().__init__()
        (
            self
            .background_color('#ffde21')
            .border_radius('4px')
            .padding('10px')
        )


class CyanBoxPyChain(StyleComponent):
    def __init__(self) -> None:
        super().__init__()

        self.background_color('#11dcdc')\
            .border_radius('4px')\
            .padding('10px')


