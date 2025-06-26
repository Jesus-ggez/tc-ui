#.?
from stubs.initializer import init_context
from stubs.reader import Reader
from stubs.node import Node
from stubs.data import Pyo


#<·
def main() -> None:
    init_context()
    Node.read_node(
        action=Reader,
    )

    for k, v in Pyo.params.items():
        vv = set(
            i.strip()
            for i in v
        )
        print(
            k, len(vv)
        )


if __name__ == '__main__':
    main()
