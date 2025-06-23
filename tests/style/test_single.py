from tc_ui import StyleComponent
import pytest

#<·

# --- Factory functions ---
def create_style_with_padding() -> StyleComponent:
    """Creates a StyleComponent with padding using method chaining."""
    return StyleComponent().padding('10px')

def create_style_with_explicit_padding() -> StyleComponent:
    """Creates a StyleComponent with padding via explicit properties dict."""
    return StyleComponent({'padding': '10px'})

# --- Test subclasses ---
class BasicPaddedStyle(StyleComponent):
    """Basic subclass that adds padding in __init__."""
    def __init__(self) -> None:
        super().__init__()
        self.padding('10px')

class ExplicitPaddedStyle(StyleComponent):
    """Subclass with explicit padding in constructor."""
    def __init__(self) -> None:
        super().__init__({'padding': '10px'})

class SelfAssignedPaddedStyle(StyleComponent):
    """Subclass that assigns padding after initialization."""
    def __init__(self) -> None:
        super().__init__()
        self.properties = {'padding': '10px'}

class ParametrizedPaddedStyle(StyleComponent):
    """Subclass with parameters but no effect on padding."""
    def __init__(self, foo: str = '') -> None:
        super().__init__()
        self.padding('10px')

class ParametrizedWithFieldPaddedStyle(StyleComponent):
    """Subclass with parameters stored as field and padding."""
    def __init__(self, foo: str = '') -> None:
        super().__init__()
        self.foo = foo
        self.padding('10px')

class NestedPaddedStyle(BasicPaddedStyle):
    """Subclass of BasicPaddedStyle with additional padding."""
    def __init__(self) -> None:
        super().__init__()
        self.padding('10px')  # Note: This will override the parent's padding

class NestedParametrizedPaddedStyle(BasicPaddedStyle):
    """Subclass with parameters that doesn't affect padding."""
    def __init__(self, foo: str = '') -> None:
        super().__init__()
        self.padding('10px')

class NestedSuperParametrizedPaddedStyle(ParametrizedPaddedStyle):
    """Subclass that calls super().__init__ with parameters."""
    def __init__(self) -> None:
        super().__init__('super().__init__(params)')
        self.padding('10px')

class PaddingAsParamStyle(BasicPaddedStyle):
    """Subclass that takes padding value as parameter."""
    def __init__(self, param: str = '10px') -> None:
        super().__init__()
        self.padding(param)

class UpdatePaddingStyle(StyleComponent):
    """Subclass that uses update() method to set padding."""
    def __init__(self, param: str = '10px') -> None:
        super().__init__()
        self.update({'padding': param})

# --- Expected outputs ---
EXPECTED_STR = '{\n\tpadding: 10px;\n}'
EXPECTED_CLASS = ' ' + EXPECTED_STR
EXPECTED_INLINE_ATTR = 'style="padding: 10px;"'
EXPECTED_INLINE = 'padding: 10px;'
EXPECTED_PROPERTIES = {'padding': '10px'}

# --- Test cases ---
@pytest.mark.parametrize("component_factory", [
    NestedSuperParametrizedPaddedStyle,
    create_style_with_explicit_padding,
    ParametrizedWithFieldPaddedStyle,
    NestedParametrizedPaddedStyle,
    create_style_with_padding,
    SelfAssignedPaddedStyle,
    ParametrizedPaddedStyle,
    ExplicitPaddedStyle,
    UpdatePaddingStyle,
    NestedPaddedStyle,
    BasicPaddedStyle,
    lambda: PaddingAsParamStyle(param='10px'),
    lambda: UpdatePaddingStyle(param='10px'),
    lambda: PaddingAsParamStyle(),
])
def test_padded_style_component(component_factory):
    """Test that all style components with padding behave correctly."""
    component = component_factory()

    # String representations
    assert str(component) == EXPECTED_STR
    assert component.__str__() == EXPECTED_STR

    # CSS output formats
    assert component.as_class('') == EXPECTED_CLASS
    assert component.as_inline(use_attr=True) == EXPECTED_INLINE_ATTR
    assert component.as_inline() == EXPECTED_INLINE

    # Properties check
    assert component.properties == EXPECTED_PROPERTIES

# Special case test for parameterized padding
def test_variable_padding():
    """Test that padding can be set with different values."""
    for padding_value in ['5px', '1em', '2rem 4rem']:
        component = PaddingAsParamStyle(param=padding_value)
        expected_str = f'{{\n\tpadding: {padding_value};\n}}'

        assert str(component) == expected_str
        assert component.as_inline() == f'padding: {padding_value};'
        assert component.properties == {'padding': padding_value}
