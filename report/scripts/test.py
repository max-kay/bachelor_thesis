import drawsvg as draw
from utils import ARROW_VARIANTS, arrow, make_legend

img = draw.Drawing(400, 500)

img.append(make_legend(50, 50, zip(ARROW_VARIANTS, range(len(ARROW_VARIANTS)))))
img.save_svg("out.svg")
