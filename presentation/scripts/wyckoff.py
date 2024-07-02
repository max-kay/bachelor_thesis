import math

import drawsvg as draw
from utils import (
    DESATURATED_COLORS,
    MAIN_COLOR,
    MAIN_STROKE_WIDTH,
    MIRROR_DASHES,
    WIDTH,
    make_regular_polygon,
)

HEIGHT = WIDTH * 4 / 5
X_MARGIN = WIDTH / 10
Y_MARGIN = HEIGHT / 10

UNITCELLS = 1
OBJ_RADIUS = 20

A_LEN = (WIDTH - 2 * X_MARGIN) / UNITCELLS
B_LEN = (HEIGHT - 2 * Y_MARGIN) / UNITCELLS


def get_base() -> draw.Drawing:
    img = draw.Drawing(WIDTH, HEIGHT)
    # veritcal lines
    for i in range(UNITCELLS + 1):
        img.append(
            draw.Line(
                X_MARGIN + i * A_LEN,
                Y_MARGIN,
                X_MARGIN + i * A_LEN,
                HEIGHT - Y_MARGIN,
                stroke=MAIN_COLOR,
                stroke_width=MAIN_STROKE_WIDTH / 2,
            )
        )
    # horizontal lines
    for i in range(UNITCELLS + 1):
        img.append(
            draw.Line(
                X_MARGIN,
                Y_MARGIN + i * B_LEN,
                WIDTH - X_MARGIN,
                Y_MARGIN + i * B_LEN,
                stroke=MAIN_COLOR,
                stroke_width=MAIN_STROKE_WIDTH / 2,
            )
        )
    # glide mirror lines
    for i in range(2 * UNITCELLS + 1):
        img.append(
            draw.Line(
                X_MARGIN + i * A_LEN / 2,
                Y_MARGIN,
                X_MARGIN + i * A_LEN / 2,
                HEIGHT - Y_MARGIN,
                stroke=DESATURATED_COLORS[0],
                stroke_width=MAIN_STROKE_WIDTH,
                stroke_dasharray=MIRROR_DASHES,
            )
        )
    # mirror lines
    for i in range(2 * UNITCELLS):
        img.append(
            draw.Line(
                X_MARGIN,
                Y_MARGIN + i * B_LEN / 2 + B_LEN / 4,
                WIDTH - X_MARGIN,
                Y_MARGIN + i * B_LEN / 2 + B_LEN / 4,
                stroke=DESATURATED_COLORS[1],
                stroke_width=MAIN_STROKE_WIDTH,
            )
        )
    # rotation centers
    base_rad = A_LEN / 15
    for i in range(2 * UNITCELLS + 1):
        for j in range(2 * UNITCELLS + 1):
            img.append(
                draw.Ellipse(
                    X_MARGIN + i * A_LEN / 2,
                    Y_MARGIN + j * B_LEN / 2,
                    1 / 2 * base_rad,
                    base_rad,
                    fill=DESATURATED_COLORS[2],
                )
            )

    return img


def p2mg():
    x_offset = A_LEN / 5
    y_offset = B_LEN / 4
    img = get_base()
    img.append(
        make_regular_polygon(
            X_MARGIN + A_LEN - x_offset,
            Y_MARGIN + y_offset,
            3,
            OBJ_RADIUS * 1.5,
            fill="red",
        )
    )
    img.append(
        make_regular_polygon(
            X_MARGIN + x_offset,
            Y_MARGIN + B_LEN - y_offset,
            3,
            OBJ_RADIUS * 1.5,
            theta_0=math.pi,
            fill="red",
        )
    )

    x_offset = A_LEN / 3
    y_offset = A_LEN / 10

    rot = 0.8

    img.append(
        make_regular_polygon(
            X_MARGIN + x_offset,
            Y_MARGIN + y_offset,
            5,
            OBJ_RADIUS * 1.5,
            fill="red",
            theta_0=rot,
        )
    )
    img.append(
        make_regular_polygon(
            X_MARGIN + x_offset,
            Y_MARGIN + B_LEN / 2 - y_offset,
            5,
            OBJ_RADIUS * 1.5,
            fill="red",
            theta_0=-rot,
        )
    )

    img.append(
        make_regular_polygon(
            X_MARGIN + A_LEN - x_offset,
            Y_MARGIN + B_LEN - y_offset,
            5,
            OBJ_RADIUS * 1.5,
            theta_0=rot + math.pi,
            fill="red",
        )
    )
    img.append(
        make_regular_polygon(
            X_MARGIN + A_LEN - x_offset,
            Y_MARGIN + B_LEN / 2 + y_offset,
            5,
            OBJ_RADIUS * 1.5,
            theta_0=-(rot + math.pi),
            fill="red",
        )
    )

    img.save_svg("figs/wyckoff.svg")


def make_rectangle(x, y, w, h) -> draw.Path:
    path = draw.Path()
    path.M(x + w / 2, y + h / 2)
    path.l(-w, 0)
    path.l(0, -h)
    path.l(w, 0)
    path.l(0, h)
    return path
