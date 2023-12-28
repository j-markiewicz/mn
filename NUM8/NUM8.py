from random import random
import numpy as np
import matplotlib.pyplot as plt
import sys


def f(a, b, c, d):
    return lambda x: a * x**2 + b * np.sin(x) + c * np.cos(5 * x) + d * np.exp(-x)


def g(a, b, c, d):
    return lambda x: a * x**2 + b * x**3 + c * np.log(x) + d * np.sin(x)


def approximate(xs, ys, fn):
    assert len(xs) == len(ys)

    def R(x, y, fn):
        return lambda beta: y - fn(*beta)(x)

    r = [R(xs[i], ys[i], fn) for i in range(len(xs))]
    β = np.zeros(4)

    for _ in range(1000):
        J = np.zeros((len(r), len(β)))
        for i in range(len(J)):
            for j in range(len(J[i])):
                dβ = np.zeros_like(β)
                dβ[j] = 1e-9
                J[i][j] = (r[i](β + dβ) - r[i](β)) / dβ[j]

        Δ = np.linalg.solve(
            J.T @ J,
            -(J.T @ np.array([ri(β) for ri in r]))
        )
        β += Δ

        if np.linalg.norm(Δ) < 1e-9:
            break

    print("R^2 = ", sum([ri(β) ** 2 for ri in r]))

    return β


def read_data(filename):
    import csv

    data = ([], [])
    with open(filename, newline="") as csvfile:
        spamreader = csv.reader(csvfile, delimiter=",", quotechar='"')
        for row in spamreader:
            data[0].append(float(row[0]))
            data[1].append(float(row[1]))
    return np.array(data[0]), np.array(data[1])


B_ARGS = (
    (random() - 0.5),
    (random() - 0.5) * 2,
    (random() - 0.5) * 10,
    (random() - 0.5) * 50,
)
FNS = {"a": f, "b": g}
DATA = {
    "a": read_data("./data.csv"),
    "b": (
        list(np.logspace(1, 20, 20, base=1.1)),
        list(
            map(
                lambda y: y + (random() - 0.5),
                g(*B_ARGS)(np.logspace(1, 20, 20, base=1.1)),
            )
        ),
    ),
}


if len(sys.argv) != 2:
    print("USAGE: `[komenda] a` lub `[komenda]` b")
    exit(1)


plt.style.use("fivethirtyeight")


fig, ax = plt.subplots()
fig.set_facecolor("#00000000")
ax.set_xlabel("x")
ax.set_ylabel("f(x)")

fn = FNS[sys.argv[1]]
data = DATA[sys.argv[1]]
space = np.linspace(min(data[0]), max(data[0]), 1000)

a, b, c, d = approximate(data[0], data[1], fn)
print(f"a = {a}")
print(f"b = {b}")
print(f"c = {c}")
print(f"d = {d}")

if sys.argv[1] == "b":
    print(f"prawdziwe a = {B_ARGS[0]}")
    print(f"prawdziwe b = {B_ARGS[1]}")
    print(f"prawdziwe c = {B_ARGS[2]}")
    print(f"prawdziwe d = {B_ARGS[3]}")
    ax.plot(space, fn(*B_ARGS)(space), "g--")

ax.plot(space, fn(a, b, c, d)(space))
ax.plot(data[0], data[1], ".")

plt.show()
