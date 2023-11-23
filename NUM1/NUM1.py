import matplotlib.pyplot as plt
import numpy as np
import sys


def approx_derivative(f, h, x):
    """
    Asymetryczne przybliżenie pochodnej funkcji `f` dla danego `h` przy argumencie `x`
    """
    return (f(x + h) - f(x)) / h


def approx_symmetrical_derivative(f, h, x):
    """
    Symetryczne przybliżenie pochodnej funkcji `f` dla danego `h` przy argumencie `x`
    """
    return (f(x + h) - f(x - h)) / (2 * h)


def fun(x):
    """
    Funkcja, dla której analizujemy błąd przybliżenia pochodnej
    """
    return np.sin(x**2)


def fun_derivative(x):
    """
    Dokładna pochodna funkcji, dla której analizujemy błąd przybliżenia pochodnej
    """
    return 2 * x * np.cos(x**2)


def error(f, fd, h, x, t):
    """
    Obliczenie błędu asymetrycznego przybliżenia pochodnej dla funkcji `f` o
    pochodnej `fd` dla danego `h` przy argumencie `x` dla typu `t`
    """
    return t(np.abs(t(approx_derivative(f, t(h), t(x))) - t(fd(t(x)))))


def error_symm(f, fd, h, x, t):
    """
    Obliczenie błędu symetrycznego przybliżenia pochodnej dla funkcji `f` o
    pochodnej `fd` dla danego `h` przy argumencie `x` dla typu `t`
    """
    return t(np.abs(t(approx_symmetrical_derivative(f, t(h), t(x))) - t(fd(t(x)))))


comic = False
if "--comic" in sys.argv:
    comic = True

try:
    samples = int(next(filter(lambda a: a.startswith("--samples="), sys.argv))[10:])
except:
    samples = 2000

space = np.logspace(3, -18, samples)

err_single = list(
    map(lambda h: (h, error(fun, fun_derivative, h, 0.2, np.single)), space.tolist())
)
err_double = list(
    map(lambda h: (h, error(fun, fun_derivative, h, 0.2, np.double)), space.tolist())
)

min_err_single = min(err_single, key=lambda e: e[1])
min_err_double = min(err_double, key=lambda e: e[1])

max_err_single = max(err_single, key=lambda e: e[1])
max_err_double = max(err_double, key=lambda e: e[1])

err_symm_single = list(
    map(
        lambda h: (h, error_symm(fun, fun_derivative, h, 0.2, np.single)),
        space.tolist(),
    )
)
err_symm_double = list(
    map(
        lambda h: (h, error_symm(fun, fun_derivative, h, 0.2, np.double)),
        space.tolist(),
    )
)

min_err_symm_single = min(err_symm_single, key=lambda e: e[1])
min_err_symm_double = min(err_symm_double, key=lambda e: e[1])

max_err_symm_single = max(err_symm_single, key=lambda e: e[1])
max_err_symm_double = max(err_symm_double, key=lambda e: e[1])

if comic:
    plt.xkcd()
    plt.figure().set_facecolor("#ffffffff")
    plt.gca().spines[["top", "right"]].set_visible(False)
    plt.tick_params(
        which="both",
        bottom=False,
        top=False,
        labelbottom=False,
        left=False,
        right=False,
        labelleft=False,
    )

if not comic:
    plt.style.use("fivethirtyeight")
    plt.figure().set_facecolor("#00000000")

plt.gca().set_xlabel("h")
plt.gca().set_ylabel("E(Df)")

plt.loglog(
    list(map(lambda e: e[0], err_symm_double)),
    list(map(lambda e: e[1], err_symm_double)),
    "c",
)
plt.loglog(
    list(map(lambda e: e[0], err_symm_single)),
    list(map(lambda e: e[1], err_symm_single)),
    "m",
)

plt.loglog(
    list(map(lambda e: e[0], err_double)), list(map(lambda e: e[1], err_double)), "b"
)
plt.loglog(
    list(map(lambda e: e[0], err_single)), list(map(lambda e: e[1], err_single)), "r"
)

if not comic:
    plt.legend(["symetryczna double", "symetryczna float", "double", "float"])
    plt.annotate(
        f"najmniejszy błąd dla float ≈ {min_err_single[1]:.4}",
        xy=min_err_single,
        arrowprops=dict(arrowstyle="->", color="black"),
        xytext=(6e-13, 0.001),
    )
    plt.annotate(
        f"najmniejszy błąd dla double ≈ {min_err_double[1]:.4}",
        xy=min_err_double,
        arrowprops=dict(arrowstyle="->", color="black"),
        xytext=(1e-19, 1e-10),
    )
    plt.annotate(
        f"dla symmetrycznego przybliżenia float ≈ {min_err_symm_single[1]:.4}",
        xy=min_err_symm_single,
        arrowprops=dict(arrowstyle="->", color="black"),
        xytext=(6e-13, 0.0002),
    )
    plt.annotate(
        f"dla symmetrycznego przybliżenia double ≈ {min_err_symm_double[1]:.4}",
        xy=min_err_symm_double,
        arrowprops=dict(arrowstyle="->", color="black"),
        xytext=(1e-19, 2e-11),
    )

print(
    f"minimalny błąd dla float: {min_err_single[1]:.1000} dla h = {min_err_single[0]:.1000}"
)
print(
    f"minimalny błąd dla double: {min_err_double[1]:.1000} dla h = {min_err_double[0]:.1000}"
)
print(
    f"minimalny błąd dla symetrycznego float: {min_err_symm_single[1]:.1000} dla h = {min_err_symm_single[0]:.1000}"
)
print(
    f"minimalny błąd dla symetrycznego double: {min_err_symm_double[1]:.1000} dla h = {min_err_symm_double[0]:.1000}"
)

plt.show()
