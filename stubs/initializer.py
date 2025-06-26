import os


#.?
from .fmtr import firs_alnum
from .node import Node
from .data import Pyo


#<·
def init_context() -> None:
    directory: list[str] = os.listdir()

    if not 'src' in directory:
        return

    Node.move_node('src')

    directory.clear()
    directory.extend(os.listdir())
    if not 'lib.rs' in directory:
        return


    with open('lib.rs', 'r') as doc:
        raw_content: list[str] = [l for l in doc if 'add_class' in l]

        mid_content: list[str] = [
            firs_alnum(
                s=word[::-1]
            )[::-1]
            for word in raw_content
        ]

        for content in mid_content:
            Pyo.params[content] = []
