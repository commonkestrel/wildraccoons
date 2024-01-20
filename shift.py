import svg
import random

CANVAS_WIDTH = 400
CANVAS_HEIGHT = 400
RECT_WIDTH = 20
RECT_HEIGHT = 40

canvas = svg.SVG(
    width=CANVAS_WIDTH,
    height=CANVAS_HEIGHT,
    elements=[
        svg.Style(type="text/css", text=".filled { fill: #dff }")
    ]
)

for x in range(int(CANVAS_WIDTH / RECT_WIDTH)):
    group = svg.G(class_="vertical", elements=[
        svg.AnimateTransform(attributeName="transform", type="translate", repeatCount="indefinite", dur="15s", from_="0,-160" if x % 2 == 0 else "0,0", to="0,0" if x % 2 == 0 else "0,-160")
    ])
    for y in range(int(CANVAS_HEIGHT / RECT_HEIGHT)):
        rect = svg.Rect(
            x=x*RECT_WIDTH,
            y=y*RECT_HEIGHT * 2 + x,
            width=RECT_WIDTH,
            height=RECT_HEIGHT,
            class_="filled"
        )
        group.elements.append(rect)
    canvas.elements.append(group)
print(canvas)
