import math
from colorsys import hsv_to_rgb

import drawsvg as draw

WIDTH = 500
MAIN_STROKE_WIDTH = 2
ARROW_HEAD_FACTOR = 4
MIRROR_DASHES = "2 2"

MAIN_COLOR = "black"

NUM_COLORS = 5

DESATURATED_COLORS = []


for i in range(NUM_COLORS):
    color = hsv_to_rgb(2 * i / NUM_COLORS * math.pi + 0.327429, 0.40, 0.80)
    color = "#" + "".join(map(lambda x: hex(int(256 * x))[2:], color))
    DESATURATED_COLORS.append(color)

SATURATED_COLORS = []

for i in range(NUM_COLORS):
    color = hsv_to_rgb(2 * i / NUM_COLORS * math.pi, 0.80, 0.80)
    color = "#" + "".join(map(lambda x: hex(int(256 * x))[2:], color))
    SATURATED_COLORS.append(color)

DASHES = [
    "none",
    "5 1",
    "5 1 1 1",
]
ARROW_VARIANTS = []

for d in DASHES:
    for c in SATURATED_COLORS:
        ARROW_VARIANTS.append({"color": c, "dashes": d})


def arrow(
    x_start: float,
    y_start: float,
    x_end: float,
    y_end: float,
    color=MAIN_COLOR,
    dashes="none",
    width: float = MAIN_STROKE_WIDTH / 2,
) -> draw.Group:
    angle = math.atan2(
        y_end - y_start,
        x_end - x_start,
    )
    offset = to_cartesian(width * ARROW_HEAD_FACTOR, angle)

    group = draw.Group()

    path = draw.Path(
        fill="none", stroke=color, stroke_width=width, stroke_dasharray=dashes
    )
    path.M(x_start, y_start)
    path.L(
        x_end - offset[0] * 3 / 2,
        y_end - offset[1] * 3 / 2,
    )
    group.append(path)

    triangle = make_regular_polygon(
        x_end - offset[0],
        y_end - offset[1],
        3,
        width * ARROW_HEAD_FACTOR,
        angle,
        fill=color,
    )
    group.append(triangle)
    return group


def ouroboros(
    x: float,
    y: float,
    color: str = MAIN_COLOR,
    dashes="none",
    angle: float = 0.0,
    width: float = MAIN_STROKE_WIDTH / 2,
) -> draw.Group:
    start_offset = to_cartesian(width * ARROW_HEAD_FACTOR, angle + math.pi / 4)
    end_offset = to_cartesian(width * ARROW_HEAD_FACTOR * 3 / 2, angle - math.pi / 4)
    group = draw.Group()

    path = draw.Path(
        fill="none", stroke=color, stroke_width=width, stroke_dasharray=dashes
    )

    path.M(x - start_offset[0], y - start_offset[1])
    c1 = to_cartesian(width * ARROW_HEAD_FACTOR * 6, angle + math.pi / 4)
    c2 = to_cartesian(width * ARROW_HEAD_FACTOR * 6, angle - math.pi / 4)
    path.c(
        -c1[0],
        -c1[1],
        -c2[0],
        -c2[1],
        3 / 2 * (start_offset[0] - end_offset[0]),
        3 / 2 * (start_offset[1] - end_offset[1]),
    )
    group.append(path)
    triangle = make_regular_polygon(
        x - end_offset[0],
        y - end_offset[1],
        3,
        width * ARROW_HEAD_FACTOR,
        angle,
        fill=color,
    )

    group.append(triangle)
    return group


def arc_arrow(
    x_start: float,
    y_start: float,
    x_end: float,
    y_end: float,
    flip: bool = False,
    bow: float = 1 / 5,
    color=MAIN_COLOR,
    dashes="none",
    width: float = MAIN_STROKE_WIDTH / 2,
) -> draw.Group:
    group = draw.Group()
    del_x = x_end - x_start
    del_y = y_end - y_start
    normal_x = del_y
    normal_y = -del_x
    if not flip:
        x_c = x_start + del_x / 2 + normal_x * bow
        y_c = y_start + del_y / 2 + normal_y * bow
    else:
        x_c = x_start + del_x / 2 - normal_x * bow
        y_c = y_start + del_y / 2 - normal_y * bow
    angle = math.atan2(
        y_end - y_c,
        x_end - x_c,
    )
    offset = to_cartesian(width * ARROW_HEAD_FACTOR, angle)

    path = draw.Path(
        fill="none", stroke=color, stroke_width=width, stroke_dasharray=dashes
    )
    path.M(x_start, y_start)
    path.Q(
        x_c,
        y_c,
        x_end - offset[0] * 3 / 2,
        y_end - offset[1] * 3 / 2,
    )
    group.append(path)
    triangle = make_regular_polygon(
        x_end - offset[0],
        y_end - offset[1],
        3,
        width * ARROW_HEAD_FACTOR,
        angle,
        fill=color,
    )

    group.append(triangle)
    return group


def to_cartesian(r: float, theta: float) -> tuple[float, float]:
    return (r * math.cos(theta), r * math.sin(theta))


def make_regular_polygon(
    x: float, y: float, n: int, radius: float, theta_0: float = 0, fill=MAIN_COLOR
) -> draw.Path:
    path = draw.Path(fill=fill)
    start = to_cartesian(radius, theta_0)
    path.M(x + start[0], y + start[1])
    for i in range(1, n):
        p = to_cartesian(radius, theta_0 + 2 * math.pi * (i / n))
        path.L(x + p[0], y + p[1])
    return path
