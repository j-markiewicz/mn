import matplotlib.pyplot as plt
import matplotlib.animation as animation
import numpy as np
import sys

F = lambda x: 1 / (1 + 50 * x**2)

XA = lambda n: lambda i: -1 + 2 * (i / (n + 1))
XB = lambda n: lambda i: np.cos(np.pi * (2 * i + 1) / (2 * (n + 1)))


def interpolate(n, f, xs):
    x = xs(np.array(range(0, n + 1)))
    y = f(x)
    pows = np.array(
        [np.array([xs(j) ** i for i in range(n + 1)]) for j in range(n + 1)]
    )
    a = np.linalg.solve(pows, y)
    return [lambda x: sum([a[i] * x**i for i in range(len(a))]), a]


space = np.linspace(-1, 1, 1000)

plt.style.use("fivethirtyeight")

if len(sys.argv) < 2:
    fig, ax = plt.subplots()

    fig.set_facecolor("#00000000")
    ax.set_xlabel("x")
    ax.set_ylabel("f(x), w(n, x)")

    ax.plot(space, F(space), "--")

    plt.tight_layout()
    plt.autoscale(False)

    plots = []
    for n in range(1, 40):
        plots.append(
            ax.plot(
                space,
                interpolate(n, F, XA(n))[0](space),
                "b",
                space,
                interpolate(n, F, XB(n))[0](space),
                "g",
            )
        )

    anim = animation.ArtistAnimation(fig, plots)
    plt.show()
    anim.save("./plot.webp", writer="pillow")
elif sys.argv[1] in ["a", "b"]:
    plt.figure().set_facecolor("#00000000")
    plt.gca().set_xlabel("x")
    plt.gca().set_ylabel("f(x), w(n, x)")
    plt.plot(space, F(space), "k--")
    plt.tight_layout()
    plt.autoscale(False)

    x = {"a": XA, "b": XB}[sys.argv[1]]
    for n in [3, 5, 10, 30]:
        w = interpolate(n, F, x(n))
        plt.plot(
            space,
            w[0](space),
            label=f"n = {n}",
        )

        print(f"W[{n}](x) = " + " + ".join(reversed([f"{w[1][i]}x^{i}" for i in range(len(w[1]))])).replace(" + -", " - "))
    plt.legend()
    plt.plot(space, F(space), "k--")
    plt.show()
