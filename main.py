from tc_ui import StaticComponent

a = (
    StaticComponent()
    .set_background('color')
    .set_color('bgcol')
    .set_padding('other')
    .set_margin('12px')

)

def MyComponent() -> StaticComponent:
    return (StaticComponent()
        .set_background('color')
        .set_color('bgcol')
        .set_padding('other')
        .set_margin('12px')
    )


for item in range(12):
    print(MyComponent().properties)
