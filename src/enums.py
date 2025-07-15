from enum import Enum


#<···>
class AlignItems(Enum):
    START = 'flex-start'
    STRETCH = 'stretch'
    CENTER = 'center'
    END = 'flex-end'


class AlignContent(Enum):
    SPACE_BETWEEN = 'space-between'
    SPACE_AROUND = 'space-around'
    START = 'flex-start'
    STRETCH = 'stretch'
    CENTER = 'center'
    END = 'flex-end'


class JustifyContent(Enum):
    SPACE_BETWEEN = 'space-between'
    SPACE_AROUND = 'space-around'
    SPACE_EVENLY = 'space-evenly'
    START = 'flex-start'
    CENTER = 'center'
    END = 'flex-end'
