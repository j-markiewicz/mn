import matplotlib.pyplot as plt
import numpy as np
import sys


M = np.array(
    [
        [8, 1, 0, 0],
        [1, 7, 2, 0],
        [0, 2, 6, 3],
        [0, 0, 3, 5],
    ]
)


def a(n):
    plt.gca().set_yscale("log")
    plt.gca().set_xlabel("k")
    plt.gca().set_ylabel("||y_k-1 - y_k||")

    (eigval, eigvec) = np.linalg.eig(M)
    (eigval, eigvec) = sorted(
        list(zip(abs(eigval), abs(np.transpose(eigvec)))), key=lambda x: x[0]
    )[-1]

    def iter(y):
        # ~y_k = Ay_{k-1}
        next = M @ y
        # y_k = \frac{~y_k}{||~y_k||}
        return next / np.linalg.norm(next)

    for j in range(n):
        y = np.random.rand(4)
        next = np.zeros_like(y)
        ys = [y]
        for i in range(1000):
            next = iter(y)
            ys.append(next)
            if np.linalg.norm(y - next) < 1e-10:
                λ = sum((M @ ys[-1]) / ys[-1]) / len(ys[-1])
                assert abs(λ - eigval) < 1e-5
                assert np.linalg.norm(ys[-1] - eigvec) < 1e-5
                print(f"y_{j + 1}_{i + 1} = {ys[-1]}, λ = {λ}")
                break
            y = next

        plt.plot(list(map(np.linalg.norm, map(lambda a, b: a - b, ys[:-2], ys[1:]))))


def b():
    fig, ax1 = plt.subplots()
    fig.set_facecolor("#00000000")
    ax1.set_xlabel("i")
    ax1.set_ylabel("A_i_(j,j)")
    ax2 = ax1.twinx()
    ax2.set_ylabel("A_i_(j,j-k)")
    ax2.tick_params(axis='y', grid_linestyle="--")
    ax2.set_yscale("log")

    (eigval, _) = np.linalg.eig(M)

    def iter(a):
        # A_k = Q_k R_k
        (q, r) = np.linalg.qr(a)
        # A_{k+1} = R_k Q_k
        return r @ q

    a = M
    next = np.zeros_like(a)
    matrices = [a]
    for i in range(1000):
        next = iter(a)
        matrices.append(next)
        if np.linalg.norm(a.diagonal() - next.diagonal(), np.inf) < 1e-10:
            assert all(map(lambda a, b: abs(a - b) < 1e-5, sorted(matrices[-1].diagonal()), sorted(eigval)))
            print(f"A_{i + 1} =\n{matrices[-1]}")
            break
        a = next

    ax2.plot(list(map(lambda d: abs(d[3][0]), matrices)), "--")
    ax2.plot(list(map(lambda d: abs(d[2][0]), matrices)), "--")
    ax2.plot(list(map(lambda d: abs(d[3][1]), matrices)), "--")
    ax2.plot(list(map(lambda d: abs(d[1][0]), matrices)), "--")
    ax2.plot(list(map(lambda d: abs(d[2][1]), matrices)), "--")
    ax2.plot(list(map(lambda d: abs(d[3][2]), matrices)), "--")

    for j in range(len(matrices[-1])):
        ax1.plot(list(map(lambda d: d[j], map(np.ndarray.diagonal, matrices))))


plt.style.use("fivethirtyeight")

if len(sys.argv) < 2:
    print("USAGE: py[thon3] NUM6.py a/b/c")
elif sys.argv[1] == "a":
    plt.figure().set_facecolor("#00000000")
    a(5)
    plt.show()
elif sys.argv[1] == "b":
    b()
    plt.show()
elif sys.argv[1] == "c":
    import webbrowser

    webbrowser.open(
        "http://galaxy.agh.edu.pl/~chwiej/mn/wyk/diagonalizacja_22_23.pdf#page=11"
    )
