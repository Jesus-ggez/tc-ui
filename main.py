from tc_ui import HtmlElement, StyleComponent
import sys

class MyHtElement:
    def __init__(self) -> None:
        self.properties: dict = {}

    def set_class(self, value: str):
        self.properties['class'] = value
        return self

    def as_tag(self) -> str:
        return f'<div class="red-box" style="{self.properties['style']}" ></div>\n'

    def style(self, value: str):
        self.properties['style'] = value
        return self


class MyComponent(HtmlElement):
    def __init__(self) -> None:
        super().__init__()

        self.style(value=self.Sitem().inline(False))
        self.set_class('red-box')
        self.href('docs-tring')

        self.components = [
            HtmlElement()
            .set_class('red-box')
            .style(value=self.Sitem().inline(False))
            .as_tag()
            for _ in range(10)
        ]

        comps: list = [
            MyHtElement().set_class('red-box')
            .style(value=self.Sitem().inline(False))
            .as_tag()
            for _ in range(10)
        ]

        a = obtener_tamaño_total(self.components)
        b = obtener_tamaño_total(comps)
        print(
            'rs ->', a,
        )
        print(
            'py ->', b,
        )
        print(
            'rs ->', self.components[0] + '\n',
        )
        print(
            'py ->', comps[0] + '\n'
        )

    def Sitem(self) -> StyleComponent:
        return (
            StyleComponent()
            .background_color('#dc143c')
            .border_radius('4px')
            .padding('10px')
        )

def obtener_tamaño_total(obj, seen=None):
    """Recursivamente encuentra el tamaño de un objeto y todos sus contenidos"""
    size = sys.getsizeof(obj)
    if seen is None:
        seen = set()

    obj_id = id(obj)
    if obj_id in seen:
        return 0

    seen.add(obj_id)

    if isinstance(obj, (list, tuple, set, frozenset)):
        size += sum(obtener_tamaño_total(i, seen) for i in obj)
    elif isinstance(obj, dict):
        size += sum(obtener_tamaño_total(k, seen) + obtener_tamaño_total(v, seen) for k, v in obj.items())

    return size

# Ejemplo de uso

if __name__ == '__main__':
    a = MyComponent()
    #print(f"Tamaño total de la lista compleja: {obtener_tamaño_total(a)} bytes")


    """
    print(
        'str ->',
        str(a),
        '\n'
    )
    print(
        'formated ->',
        a.formated(),
        '\n'
    )

    """
