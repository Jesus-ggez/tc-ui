from tc_ui import Text


#<·
class SimpleFromInit(Text):
    def __init__(self, css_class: str)-> None:
        super().__init__(
            value='Simple from __init__',
            tag='h1',
        )
        self.set_class(css_class)


class SimpleAdd(Text):
    def __init__(self, css_class: str) -> None:
        super().__init__()

        self.value = 'Simple explicit add'
        self.tag = 'h1'
        self.set_class(css_class)


class SimpleOnlyValueInit(Text):
    def __init__(self, css_class: str) -> None:
        super().__init__(
            value='Simple explicit value',
        )
        self.tag = 'h1'
        self.set_class(css_class)


class SimpleOnlyTagInit(Text):
    def __init__(self, css_class: str) -> None:
        super().__init__(
            tag='h1'
        )
        self.value = 'Simple explicit tag'
        self.set_class(css_class)


class SimpleWithExternParams(Text):
    def __init__(self, css_class: str, value: str = 'Extern params') -> None:
        super().__init__(
            value=value,
        )
        self.tag = 'h1'
        self.set_class(css_class)

#--#--# multiinherit
class MidInherit(SimpleFromInit):
    def __init__(self, css_class: str) -> None:
        super().__init__(css_class)
        self.value = 'Middle complex from inherit'
