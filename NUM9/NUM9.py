import matplotlib.pyplot as plt
import numpy as np
import sys


def f(x):
    return np.sin(x) - 0.4


def fd(x):
    return np.cos(x)


def g(x):
    return f(x) ** 2


def gd(x):
    return 2 * (np.sin(x) - 0.4) * np.cos(x)


def u(x):
    return g(x) / derivative(g)(x)


class MethodNotApplicableError(ValueError):
    pass


def derivative(fn):
    return {
        f: fd,
        g: gd,
        fd: lambda x: -np.sin(x),
        gd: lambda x: 2 * np.cos(x) ** 2 - 2 * (np.sin(x) - 0.4) * np.sin(x),
        u: lambda x: (derivative(g)(x) ** 2 - g(x) * derivative(derivative(g))(x))
        / derivative(g)(x) ** 2,
    }[fn]


def bisection(fn, start, end):
    startsign = np.sign(fn(start))
    endsign = np.sign(fn(end))

    if startsign == endsign:
        raise MethodNotApplicableError()

    middle = (start + end) / 2
    midsign = np.sign(fn(middle))

    if midsign == startsign:
        return (middle + end) / 2, (middle, end)
    else:
        return (start + middle) / 2, (start, middle)


def falsi(fn, start, end, last=0):
    startsign = np.sign(fn(start))
    endsign = np.sign(fn(end))

    if startsign == endsign:
        raise MethodNotApplicableError()

    between = (fn(start) * end - fn(end) * start) / (fn(start) - fn(end))
    betweensign = np.sign(fn(between))

    if betweensign == endsign:
        if last == -1:
            return (start / 2 + between) / 2, (start / 2, between, -1)
        return (start + between) / 2, (start, between, -1)
    else:
        if last == 1:
            return (between + end / 2) / 2, (between, end / 2, 1)
        return (between + end) / 2, (between, end, 1)


def secant(fn, start, end):
    line_root = end - fn(end) * (end - start) / (fn(end) - fn(start))
    return line_root, (end, line_root)


def newton(fn, x, _=...):
    res = x - fn(x) / derivative(fn)(x)
    return res, (res,)


def better(fn, x, _=...):
    if fn is not g:
        raise MethodNotApplicableError()
    return newton(u, x)


METHODS = [
    ("a (bisekcji)", bisection),
    ("b (falsi)", falsi),
    ("c (siecznych)", secant),
    ("d (newtona)", newton),
    ("5 (lepsza newtona)", better),
]
EXACT_X = np.arcsin(0.4)

if len(sys.argv) <= 1 or sys.argv[1] not in ["f", "g"]:
    print("`[komenda] f` lub `[komenda] g`")
    exit(1)

plt.gca().set_yscale("log")
plt.gca().set_xlabel("i")
plt.gca().set_ylabel("ϵ_i")

if sys.argv[1] == "f":
    for name, method in METHODS[:-1]:
        try:
            x_star, res = method(f, 0.0, np.pi / 2)
            ϵ = np.abs(EXACT_X - x_star)
            ϵs = [ϵ]

            for _ in range(1000):
                x_star, res = method(f, *res)
                ϵ = np.abs(EXACT_X - x_star)
                ϵs.append(ϵ)

                if np.abs(f(x_star)) < 1e-15:
                    break

            plt.plot(ϵs, label=name)
            print(name, ϵs)
        except MethodNotApplicableError:
            continue
elif sys.argv[1] == "g":
    for name, method in METHODS:
        try:
            x_star, res = method(g, 0.0, np.pi / 2)
            ϵ = np.abs(EXACT_X - x_star)
            ϵs = [ϵ]

            for _ in range(1000):
                x_star, res = method(g, *res)
                ϵ = np.abs(EXACT_X - x_star)
                ϵs.append(ϵ)

                if np.abs(g(x_star)) < 1e-15:
                    break

            plt.plot(ϵs, label=name)
            print(name, ϵs)
        except MethodNotApplicableError:
            continue

plt.legend()
plt.show()
