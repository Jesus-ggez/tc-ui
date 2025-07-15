#.?
from .fmtr import firs_alnum, remove_invalid_chars, replace_no_pytypes
from .data import Pyo


#<·
class Reader:
    @staticmethod
    def read_as_list(filename: str) -> list[str]:
        try:
            with open(filename, 'r') as fileno:
                return [ l.strip() for l in fileno ]

        except Exception as e:
            return []


    def __init__(self, filename: str) -> None:
        self._ignore_state: bool = False
        self._file_data: list[str] = []
        self.cls_name: str = ''
        self._filename: str = filename
        self.data: list[str] = []
        if Pyo.stop:
            return

        self.__build()


    def is_ok(self) -> bool:
        return len(self.data) > 0


    def __build(self) -> None:
        try:
            self.__read_file()
            self.create_content()
            self.format_content()

        except Exception as e:
            print(e)
            print(self.cls_name)
            print(self._filename)
            Pyo.stop = True


    def __read_file(self) -> None:
        finding: str = '#[pymethods]'

        for line in Reader.read_as_list(self._filename):
            if not line.startswith(finding):
                continue

            if line.startswith('impl'):
                self.cls_name += line.removeprefix('impl').strip().split('{')[0].strip()
                continue

            if not finding:
                self._file_data.append(line)
                continue

            finding = ''


    def _create_door(self, raw: str, activator: str) -> None:
        if activator in raw:
            self._ignore_state = True


    def ignore_dunder(self, raw: str) -> None:
        if 'fn' in raw:
            fn_name: str = firs_alnum(
                raw.split('fn')[1]
            )

            if fn_name.startswith('__') and fn_name.endswith('__'):
                self._ignore_state = True

    def _create_function(self, refer: str) -> None:
        if refer.startswith(('pub', 'fn')) and not self.is_in_ctt:
            self.is_in_ctt = True

        if not self._ignore_state and self.is_in_ctt:
            self._last_content[-1] += refer.strip()

        if '{' in refer and self.is_in_ctt:
            self._ignore_state = False
            self.is_in_ctt = False
            self._last_content.append('')


    def create_content(self) -> None:
        self._last_content: list[str] = ['']
        self.is_in_ctt = True

        for line in self._file_data:
            if 'pyo3' in line:
                continue

            self._create_door(line, '#[setter]')
            self._create_door(line, '#[getter]')
            self.ignore_dunder(line)

            self._create_function(line)


    def format_content(self) -> None:
        all_data: list[str] = []

        for line in self._last_content:
            if not line.strip():
                continue

            all_data.append('def ')

            # 'pub fn/fn', 'fn_name()'
            fn_content: str = line.split('fn')[1]

            fn_name, rest = fn_content.split('(', 1)

            fn_params, fn_return = rest.split(')', 1)

            # function name
            all_data[-1] += fn_name.strip()


            # function content
            params: str = remove_invalid_chars(fn_params)
            all_data[-1]  += f'({params}) -> '


            # function end
            end: str = remove_invalid_chars(
                replace_no_pytypes(fn_return)
            ) or 'None' + ': ...'

            all_data[-1] += end.replace('self', repr(self.cls_name))

        self.data: list[str] = all_data
        #for s in self.data: print(s)
