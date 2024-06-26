import math

import drawsvg as draw
from utils import (
    ARROW_VARIANTS,
    DESATURATED_COLORS,
    MAIN_COLOR,
    MAIN_STROKE_WIDTH,
    MIRROR_DASHES,
    WIDTH,
    arc_arrow,
    arrow,
    make_regular_polygon,
    ouroboros,
)

HEIGHT = WIDTH * 4 / 5
X_MARGIN = WIDTH / 16
Y_MARGIN = HEIGHT / 16

UNITCELLS = 3
OBJ_RADIUS = 12

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


def title():
    x_offset = A_LEN / 5
    y_offset = B_LEN / 4
    img = get_base()
    for i in range(UNITCELLS):
        for j in range(UNITCELLS):
            img.append(
                make_regular_polygon(
                    X_MARGIN + i * A_LEN + x_offset,
                    Y_MARGIN + j * B_LEN + y_offset,
                    3,
                    OBJ_RADIUS,
                    fill="red",
                )
            )
            img.append(
                make_regular_polygon(
                    X_MARGIN + (i + 1) * A_LEN - x_offset,
                    Y_MARGIN + (j + 1) * B_LEN - y_offset,
                    3,
                    OBJ_RADIUS,
                    theta_0=math.pi,
                    fill="red",
                )
            )
    x_0 = X_MARGIN + A_LEN + x_offset
    y_0 = Y_MARGIN + B_LEN + y_offset

    img.append(arc_arrow(x_0, y_0, x_0 + A_LEN, y_0, **ARROW_VARIANTS[1]))
    img.append(arc_arrow(x_0, y_0, x_0 - A_LEN, y_0, flip=True, **ARROW_VARIANTS[1]))

    img.append(
        arc_arrow(
            x_0,
            y_0,
            x_0 + A_LEN - 2 * x_offset,
            y_0 + B_LEN - 2 * y_offset,
            flip=True,
            **ARROW_VARIANTS[2],
        )
    )
    img.append(
        arc_arrow(
            x_0,
            y_0,
            x_0 + A_LEN - 2 * x_offset,
            y_0 - B_LEN + 2 * y_offset,
            **ARROW_VARIANTS[2],
        )
    )

    img.append(
        arc_arrow(
            x_0,
            y_0,
            x_0 - 2 * x_offset,
            y_0 - 2 * y_offset,
            flip=True,
            **ARROW_VARIANTS[3],
        )
    )
    img.append(
        arc_arrow(
            x_0,
            y_0,
            x_0 - 2 * x_offset,
            y_0 + 2 * y_offset,
            **ARROW_VARIANTS[3],
        )
    )

    img.append(arrow(x_0, y_0, x_0, y_0 + B_LEN, **ARROW_VARIANTS[4]))
    img.append(arrow(x_0, y_0, x_0, y_0 - B_LEN, **ARROW_VARIANTS[4]))

    img.append(ouroboros(x_0, y_0))

    img.save_svg("figs/title.svg")
