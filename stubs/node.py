import os


#.?
from .data import Pyo


#<·
class Node:
    @staticmethod
    def read_node(action) -> None:
        for f in os.listdir():
            if os.path.isfile(f):
                if ( value := action(f) ).is_ok() and value.cls_name in Pyo.params:
                    Pyo.params[value.cls_name].extend(value.data)

                continue

            if os.path.isdir(f):
                Node.move_node(f)
                if not Node.is_valid_node():
                    return


                Node.read_node(action)
                continue


    @staticmethod
    def move_node(dest: str) -> None:
        try:
            os.chdir(dest)

        except Exception as e:
            print(e)


    @staticmethod
    def is_valid_node() -> bool:
        return 'mod.rs' in os.listdir()

