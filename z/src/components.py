from tc_ui import HtmlElement


#.?
from .containers import Container
from .styles import (
    YellowBoxRsStyle,
    GreenBoxPureCls,
    CyanBoxPyChain,
    BlueBoxAddDict,
    RedBoxFromDict,
)
from .texts import (
    SimpleWithExternParams,
    SimpleOnlyValueInit,
    SimpleOnlyTagInit,
    SimpleFromInit,
    SimpleAdd,
)
from .texts import (
    MidInherit,
)


#<·
class Index(HtmlElement):
    def __init__(self) -> None:
        super().__init__(
            tag='html'
        )

        self.build()


    def build(self) -> None:
        all_boxes: list[HtmlElement] = (
            self.__test_box_yellow() +
            self.__test_box_green() +
            self.__test_box_blue() +
            self.__test_box_cyan() +
            self.__test_box_red()
        )

        self.components = [
            tag.as_tag() for tag in all_boxes
        ]
        self.__alL_styles()

    def __test_box(self, color: str) -> list:
        all_tags: list = [
            SimpleWithExternParams,
            SimpleOnlyValueInit,
            SimpleOnlyTagInit,
            SimpleFromInit,
            SimpleAdd,

            MidInherit,
        ]
        data: list = [ tag( css_class=color ) for tag in all_tags ]
        return data

    def __test_box_container(self, color: str) -> list:
        return [
            Container(
                items=self.__test_box(color=color)
            )
        ]

    def __test_box_yellow(self) -> list:
        return self.__test_box_container( color='yellow-box' )


    def __test_box_green(self) -> list:
        return self.__test_box_container( color='green-box' )


    def __test_box_blue(self) -> list:
        return self.__test_box_container( color='blue-box' )


    def __test_box_cyan(self) -> list:
        return self.__test_box_container( color='cyan-box' )


    def __test_box_red(self) -> list:
        return self.__test_box_container( color='red-box' )


    def __alL_styles(self) -> None:
        styles: list = [
            YellowBoxRsStyle().as_class('.yellow-box'),
            GreenBoxPureCls().as_class('.green-box'),
            BlueBoxAddDict().as_class('.blue-box'),
            CyanBoxPyChain().as_class('.cyan-box'),
            RedBoxFromDict().as_class('.red-box'),
        ]
        style_tag: HtmlElement = HtmlElement('style')
        for item in styles:
            style_tag.add_html(item)


        self.append(style_tag)
