import drawsvg as draw
from utils import ARROW_VARIANTS, arrow

img = draw.Drawing(400, 500)
for i, variant in enumerate(ARROW_VARIANTS):
    img.append(arrow(20, i * 10 + 20, 480, i * 10 + 20, **variant))
img.save_svg("out.svg")
