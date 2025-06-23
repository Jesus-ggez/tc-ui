from tc_ui import StyleComponent
import pytest

#<·

# --- Factory functions ---
def create_default_style() -> StyleComponent:
    """Creates a StyleComponent with default empty properties."""
    return StyleComponent()

def create_explicit_empty_style() -> StyleComponent:
    """Creates a StyleComponent with explicit empty dict."""
    return StyleComponent({})

# --- Test subclasses ---
class BasicStyleComponent(StyleComponent):
    """Basic subclass with implicit empty properties."""
    def __init__(self) -> None:
        super().__init__()

class ExplicitEmptyStyleComponent(StyleComponent):
    """Subclass with explicit empty properties."""
    def __init__(self) -> None:
        super().__init__({})

class SelfAssignedEmptyStyleComponent(StyleComponent):
    """Subclass that assigns empty properties after initialization."""
    def __init__(self) -> None:
        super().__init__()
        self.properties = {}

class ParametrizedStyleComponent(StyleComponent):
    """Subclass with additional parameters but no property changes."""
    def __init__(self, foo: str = '') -> None:
        super().__init__()

class ParametrizedStyleWithFieldComponent(StyleComponent):
    """Subclass with additional parameters stored as field."""
    def __init__(self, foo: str = '') -> None:
        super().__init__()
        self.foo = foo

class NestedStyleComponent(BasicStyleComponent):
    """Subclass of BasicStyleComponent."""
    def __init__(self) -> None:
        super().__init__()

class NestedParametrizedStyleComponent(BasicStyleComponent):
    """Subclass of BasicStyleComponent with parameters."""
    def __init__(self, foo: str = '') -> None:
        super().__init__()

class NestedSuperParametrizedStyleComponent(ParametrizedStyleComponent):
    """Subclass that calls super().__init__ with parameters."""
    def __init__(self) -> None:
        super().__init__('super().__init__(params)')

# --- Test cases ---
@pytest.mark.parametrize("component_factory", [
    NestedSuperParametrizedStyleComponent,
    ParametrizedStyleWithFieldComponent,
    NestedParametrizedStyleComponent,
    SelfAssignedEmptyStyleComponent,
    ExplicitEmptyStyleComponent,
    create_explicit_empty_style,
    ParametrizedStyleComponent,
    NestedStyleComponent,
    create_default_style,
    BasicStyleComponent,
    lambda: ParametrizedStyleComponent(foo='any'),
])
def test_empty_style_component(component_factory):
    """Test that all empty style components behave correctly."""
    component: StyleComponent = component_factory() if callable(component_factory) else component_factory()

    # String representations
    assert str(component) == '{  }'
    assert component.__str__() == '{  }'

    # CSS output formats
    assert component.as_class('') == ' {\n}'
    assert component.as_inline(use_attr=True) == 'style=""'
    assert component.as_inline() == ''

    # Properties check
    assert component.properties == {}

