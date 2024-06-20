import math

import drawsvg as draw
from utils import (
    ARROW_VARIANTS,
    DASHES,
    DESATURATED_COLORS,
    MAIN_COLOR,
    MAIN_STROKE_WIDTH,
    SATURATED_COLORS,
    WIDTH,
    arc_arrow,
    make_regular_polygon,
    ouroboros,
)

UNITCELLS = 7
HEIGHT = WIDTH / 4
CELL_LENGTH = WIDTH / (UNITCELLS + 1)
MARGIN = CELL_LENGTH / 2
TICK_HEIGHT = 40
OBJ_RADIUS = 10
MIRROR_OFFSET = 2 * CELL_LENGTH / 5


def get_base() -> draw.Drawing:
    img = draw.Drawing(WIDTH, HEIGHT)
    img.append(
        draw.Line(
            MARGIN,
            HEIGHT / 2,
            WIDTH - MARGIN,
            HEIGHT / 2,
            stroke=MAIN_COLOR,
            stroke_width=MAIN_STROKE_WIDTH,
        )
    )
    for i in range(UNITCELLS + 1):
        x = MARGIN + i * CELL_LENGTH
        img.append(
            draw.Line(
                x,
                HEIGHT / 2 + TICK_HEIGHT / 2,
                x,
                HEIGHT / 2 - TICK_HEIGHT / 2,
                stroke=MAIN_COLOR,
                stroke_width=MAIN_STROKE_WIDTH / 2,
            )
        )
    return img


def get_base_mirror() -> draw.Drawing:
    img = get_base()
    for i in range(UNITCELLS):
        x = MARGIN + MIRROR_OFFSET + i * CELL_LENGTH
        img.append(
            draw.Line(
                x,
                HEIGHT / 2 + TICK_HEIGHT / 3,
                x,
                HEIGHT / 2 - TICK_HEIGHT / 3,
                stroke=DESATURATED_COLORS[0],
                stroke_width=MAIN_STROKE_WIDTH / 2,
            )
        )
    for i in range(UNITCELLS):
        x = MARGIN + MIRROR_OFFSET + i * CELL_LENGTH + CELL_LENGTH / 2
        img.append(
            draw.Line(
                x,
                HEIGHT / 2 + TICK_HEIGHT / 3,
                x,
                HEIGHT / 2 - TICK_HEIGHT / 3,
                stroke=DESATURATED_COLORS[1],
                stroke_width=MAIN_STROKE_WIDTH / 2,
            )
        )
    return img


def connection(
    x1: float,
    x2: float,
    y_offset: float,
    above: bool,
    **kwargs,
) -> draw.Group:
    flip = True
    if above and x1 > x2:
        flip = False
    if (not above) and x1 < x2:
        flip = False
    if above:
        y2 = HEIGHT / 2 + TICK_HEIGHT / 2
        y1 = y2 + y_offset
    else:
        y2 = HEIGHT / 2 - TICK_HEIGHT / 2
        y1 = y2 - y_offset
    return arc_arrow(x1, y1, x2, y2, flip, **kwargs)


def get_r_triangle(x: float, y: float, fill=MAIN_COLOR) -> draw.Path:
    return make_regular_polygon(x, y, 3, OBJ_RADIUS, fill=fill)


def get_l_triangle(x: float, y: float, fill=MAIN_COLOR) -> draw.Path:
    return make_regular_polygon(x, y, 3, OBJ_RADIUS, theta_0=math.pi, fill=fill)


def diamond(x: float, y: float, fill=MAIN_COLOR) -> draw.Path:
    return make_regular_polygon(x, y, 4, OBJ_RADIUS, fill=fill)


def p1_example():
    feature_off_set = CELL_LENGTH / 3
    img = get_base()
    for i in range(UNITCELLS):
        x = MARGIN + feature_off_set + i * CELL_LENGTH
        img.append(get_r_triangle(x, HEIGHT / 2, fill="red"))

    origin = MARGIN + UNITCELLS // 2 * CELL_LENGTH + feature_off_set
    for i in range(-3, 3 + 1):
        if i == 0:
            continue
        arrow_variant = abs(i) - 1
        img.append(
            connection(
                origin,
                origin + CELL_LENGTH * i,
                abs(3 * i),
                i < 0,
                **ARROW_VARIANTS[arrow_variant],
            )
        )
    img.append(ouroboros(origin, HEIGHT / 2))
    img.save_svg("figs/p1.svg")


def p1m_g_example():
    feature_off_set = CELL_LENGTH / 5
    img = get_base_mirror()
    xs = []
    for i in range(UNITCELLS):
        x = MARGIN + feature_off_set + i * CELL_LENGTH
        img.append(get_r_triangle(x, HEIGHT / 2, fill="red"))
        xs.append(x)
        x = MARGIN + MIRROR_OFFSET - (feature_off_set - MIRROR_OFFSET) + i * CELL_LENGTH
        img.append(get_l_triangle(x, HEIGHT / 2, fill="red"))
        xs.append(x)
    origin_idx = len(xs) // 2
    origin = xs[origin_idx]

    # integer translations
    for i in range(-2, 2 + 1):
        if i == 0:
            continue
        img.append(
            connection(
                origin,
                xs[origin_idx + 2 * i],
                abs(3 * i),
                False,
                color=SATURATED_COLORS[abs(i) - 1],
                dashes=DASHES[0],
            )
        )
    # non integer translations
    for i, offset in enumerate([3, 1, 2, 4]):
        x = xs[origin_idx - 3 + 2 * i]
        img.append(
            connection(
                origin,
                x,
                offset * 2,
                True,
                color=SATURATED_COLORS[i],
                dashes=DASHES[1],
            )
        )

    img.save_svg("figs/p1m_g.svg")


def p1m_s_example():
    img = get_base_mirror()
    xs = []
    for i in range(UNITCELLS):
        x = MARGIN + MIRROR_OFFSET + CELL_LENGTH * i
        xs.append(x)
        img.append(diamond(x, HEIGHT / 2, fill="red"))
    origin = xs[len(xs) // 2]
    for i in range(1, 3):
        img.append(
            connection(
                origin,
                origin + i * CELL_LENGTH,
                abs(3 * i),
                bool(i % 2),
                **ARROW_VARIANTS[i - 1],
            )
        )
        img.append(
            connection(
                origin,
                origin - i * CELL_LENGTH,
                abs(3 * i),
                bool(i % 2),
                **ARROW_VARIANTS[i - 1],
            )
        )
    img.save_svg("figs/p1m_s.svg")
