
#.?
from .reader import Reader
from .node import Node


#<·
def test() -> None:
    println = lambda s: (
        #print(),
        #print('old ->', s),
        #print('new ->', remove_invalid_chars(s)),
        #print('fmt ->', remove_invalid_chars(replace_no_pytypes(s)))
    )

    example: str = 'remove_invalid_chars: <<naim<<tuple>>>, Py<&bounof<VEc<str>>>'
    exampl1: str = 'remove_invalid_chars<<<<tuple>>>, Py<&bounof<VEc<str>>>'
    exampl2: str = 'remove_invalid_chars<<<tuple>>>, Py<&bounof<VEc<str>>>'
    exampl3: str = '-> PyResult<\'_, Self>'

    println(example)
    println(exampl1)
    println(exampl2)
    println(exampl3)


#if __name__ == '__main__':
    #app()
