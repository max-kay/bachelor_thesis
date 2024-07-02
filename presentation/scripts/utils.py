import math
from colorsys import hsv_to_rgb

import drawsvg as draw
import numpy as np

FONT_SIZE = 20
WIDTH = 800
MAIN_STROKE_WIDTH = 3
ARROW_HEAD_FACTOR = 4
MIRROR_DASHES = "5 5"

MAIN_COLOR = r"#aaaaaa"

NUM_COLORS = 5

DESATURATED_COLORS = []

for i in range(NUM_COLORS):
    color = hsv_to_rgb(2 * i / NUM_COLORS * math.pi + 0.327429, 0.40, 0.65)
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

LEGEND_WIDTH = 100
LEGEND_MARGIN = 80


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


def ouroboros(
    x: float,
    y: float,
    color: str = MAIN_COLOR,
    dashes="none",
    angle: float = 0,
    width: float = MAIN_STROKE_WIDTH / 2,
) -> draw.Group:
    gap_angle = math.pi / 8
    radius = width * ARROW_HEAD_FACTOR * 3 / 2 / math.tan(gap_angle)
    pos = np.array([x, y])
    start_offset = np.array(
        to_cartesian(width * ARROW_HEAD_FACTOR * 3 / 2, angle + math.pi / 2 - gap_angle)
    )
    end_offset = np.array(
        to_cartesian(width * ARROW_HEAD_FACTOR * 3 / 2, angle - math.pi / 2 + gap_angle)
    )
    start = pos - start_offset
    end = pos - end_offset

    group = draw.Group()

    path = draw.Path(
        fill="none", stroke=color, stroke_width=width, stroke_dasharray=dashes
    )
    path.M(start[0], start[1])
    path.A(radius, radius, 0, True, False, end[0], end[1])
    group.append(path)

    triangle_center = pos - to_cartesian(
        width * ARROW_HEAD_FACTOR, angle - math.pi / 2 + gap_angle
    )
    triangle = make_regular_polygon(
        triangle_center[0],
        triangle_center[1],
        3,
        width * ARROW_HEAD_FACTOR,
        theta_0=angle - math.pi / 2 + gap_angle,
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


def make_legend(
    h_center: float,
    left: float,
    legend: list[tuple[dict, int]],
) -> draw.Group:
    length = 60
    text_offset = 20
    top = h_center - (len(legend) - 1) * FONT_SIZE * 1.5 / 2
    group = draw.Group()
    line_start = left
    line_end = left + length
    text_start = left + length + text_offset
    for i, (variant, num) in enumerate(legend):
        center_y = top + i * FONT_SIZE * 1.5
        path = draw.Path(
            fill="none",
            stroke=variant["color"],
            stroke_width=MAIN_STROKE_WIDTH / 2,
            stroke_dasharray=variant["dashes"],
        )
        path.M(line_start, center_y)
        path.L(line_end, center_y)
        group.append(path)

        text = draw.Text(
            fill="white",
            font_size=FONT_SIZE,
            text=f"{num}",
            dominant_baseline="middle",
            x=text_start,
            y=center_y,
        )
        group.append(text)
    return group
