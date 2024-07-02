import math

import drawsvg as draw
from utils import MAIN_COLOR, arc_arrow, arrow, make_regular_polygon, to_cartesian

RADIUS = 60


def generator():
    img = draw.Drawing(width=150, height=150, origin="center")
    img.append(make_regular_polygon(0, 0, 3, 60, fill="red"))
    p_0 = to_cartesian(63, 0)
    p_1 = to_cartesian(63, -math.pi / 3 * 2)
    p_2 = to_cartesian(63, math.pi / 3 * 2)
    img.append(arc_arrow(*p_0, *p_1, color=MAIN_COLOR, flip=True))
    img.save_svg("figs/generator.svg")

    img.append(arc_arrow(*p_1, *p_2, color=MAIN_COLOR, flip=True))
    img.append(arc_arrow(*p_2, *p_0, color=MAIN_COLOR, flip=True))
    img.save_svg("figs/generator2.svg")

    img = draw.Drawing(width=150, height=150)
    for i in range(3):
        img.append(
            arrow(15 + i * 40, 135 - i * 40, 15 + i * 40 + 40, 135 - i * 40 - 40)
        )
        img.save_svg(f"figs/arrow{i}.svg")
