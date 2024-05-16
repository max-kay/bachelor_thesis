import matplotlib.pyplot as plt
import numpy as np
from scipy.fft import fft2 as fft
from scipy.fft import ifft2 as ifft

UNIT = 50
GRID_SIZE = 20

x, y = np.meshgrid(np.arange(UNIT), np.arange(UNIT))
center1 = (int(0.5 * UNIT), int(0.25 * UNIT))
center2 = (int(0.5 * UNIT), int(0.75 * UNIT))
radius = int(UNIT * 0.2)
distance1 = np.sqrt((x - center1[0]) ** 2 + (y - center1[1]) ** 2)
distance2 = np.sqrt((x - center2[0]) ** 2 + (y - center2[1]) ** 2)
circle_image = np.where((distance1 <= radius) | (distance2 <= radius), 1, 0)

VERTICAL = circle_image.astype(np.float64)
HORIZONTAL = np.rot90(circle_image).astype(np.float64)


def run():
    random_grid = np.random.random(size=(GRID_SIZE, GRID_SIZE))
    for i, p in enumerate(np.linspace(0.5, 1.0, 8)):
        realspace = np.zeros((GRID_SIZE * UNIT, GRID_SIZE * UNIT), np.float64)
        for x in range(GRID_SIZE):
            for y in range(GRID_SIZE):
                if (x + y) % 2 == 0:
                    if random_grid[x, y] < p:
                        realspace[
                            x * UNIT : (x + 1) * UNIT, y * UNIT : (y + 1) * UNIT
                        ] = VERTICAL
                    else:
                        realspace[
                            x * UNIT : (x + 1) * UNIT, y * UNIT : (y + 1) * UNIT
                        ] = HORIZONTAL
                else:
                    if random_grid[x, y] < p:
                        realspace[
                            x * UNIT : (x + 1) * UNIT, y * UNIT : (y + 1) * UNIT
                        ] = HORIZONTAL
                    else:
                        realspace[
                            x * UNIT : (x + 1) * UNIT, y * UNIT : (y + 1) * UNIT
                        ] = VERTICAL

        plt.imsave(f"imgs/test{i+1}.png", realspace, cmap="gray_r")
        structure_factor = fft(realspace)
        intensities = np.abs(structure_factor) ** 2
        save_diffraction(f"imgs/fft{i+1}.png", intensities)
        plt.imsave(f"imgs/pdf{i+1}.png", np.abs(ifft(intensities)), cmap="gray_r")


def save_diffraction(path: str, diff: np.ndarray):
    DELTA = 1
    X_MAX = 1000
    B = DELTA
    A = 2 * DELTA / X_MAX

    # def f_0(x):
    #     return 1 / np.pi * np.arctan(A * x - B) + 1 / 2
    #
    # def f_1(x):
    #     new = np.sqrt(x / X_MAX)
    #     new[new > 1] = 1
    #     return new
    #
    # def f(x):
    #     return (4 * f_0(1 / x) + f_1(1 / x)) / 5

    def f(x):
        return np.power(x, 0.01)

    # fig, (ax1, ax2) = plt.subplots(2)
    # ax1.hist(diff.flatten(), bins=100, range=(0, 6000), density=True)
    #
    # xs = np.linspace(0, 6000, 100)
    # ys = f(xs)
    #
    # ax2.plot(xs, ys)
    # plt.show()

    img = f(diff)
    plt.imsave(
        path,
        img,
        cmap="gray_r",
    )


def save_diffraction2(path: str, diff: np.ndarray):
    diff[diff < 0.0001] = 0.0001
    log_intensities = np.log(diff)
    plt.imsave(
        path,
        log_intensities,
        cmap="gray",
        origin="upper",
        vmin=np.nanmin(log_intensities),
        vmax=np.nanmax(log_intensities),
    )
