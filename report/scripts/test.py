import drawsvg as draw
from utils import ARROW_VARIANTS, arrow

img = draw.Drawing(400, 500)
for i, variant in enumerate(ARROW_VARIANTS):
    img.append(arrow(20, i * 10 + 20, 480, i * 10 + 20, **variant))

path = draw.Path(stroke_width=10, stroke="black", fill="none")
path.arc(100, 100, 80, 0, 180)
img.append(path)
img.save_svg("out.svg")
