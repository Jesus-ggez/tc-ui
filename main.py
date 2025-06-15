from tc_ui import StaticComponent

a = (
    StaticComponent()
    .set_background('color')
    .set_color('bgcol')
    .set_padding('other')
    .set_margin('12px')
)

print(a.properties)
