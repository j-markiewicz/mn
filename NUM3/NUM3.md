Problem polega na rozwiązaniu równania $y = A^{-1}x$ oraz obliczenia wyznacznika macierzy $A$, z tym że macierz $A$ jest pasmowa. W ogólnym przypadku rozwiązanie równania miałoby złożoność obliczeniową $O(N^3)$. Używając struktury macierzy można rozwiązać to równanie ze złożonością $O(N)$.

# Macierz $A$
Macierz pasmową $A$ można zapisać w tablicy prostokątnej `L + U + 1` na $N$, gdzie `L` oznacza liczbę pasem pod główną diagonalą a `U` nad nią. Macierz w zadaniu jest podana jako $A_N = \begin{bmatrix} 1.2 & 0.1 & 0.15 & & 0 \\ 0.2 & 1.2 & \frac{0.1}{2} & \frac{0.15}{2^2} \\ & ... & ... & ... & ... \\ & & 0.2 & 1.2 & \frac{0.1}{N} \\ 0 & & & 0.2 & 1.2 \end{bmatrix}$ z `L = 1` i `U = 2`. Przykład macierzy dla $N = 10$ jest wyświetlony przez program.

# Rozkład LU
Dla ogólnej macierzy rozkład LU obliczany jest za pomocą równań $u_{k, m} = a_{k, m} - \sum_{j=1}^{k-1} l_{k,j} u_{j,m}$ dla $m = k, k + 1, ..., n$ i $l_{i, k} = \frac{(a_{i, k} - \sum_{j=1}^{k-1} l_{i,j} u_{j, k})}{u_{kk}}$ for $i = k + 1, k + 2, ..., n$. Dla macierzy pasmowej można te równania zoptymalizować do $u_{k,m} = a_{k,m} - \sum_{j=\max(1, k - L, m - U)}^{k-1} l_{k,j} u_{j,m}$ for $m = k, k + 1, ..., k + U \le n$ i $l_{i,k} = \frac{(a_{i,k} - \sum_{j=\max(1, i - L, k - U)}^{k-1} l_{i,j} u_{j,k})}{u_{k,k}}$ for $i = k + 1, k + 2, ..., k + L \le n$.
Ta optymalizacja pozwala na rozkład LU macierzy w czasie liniowym:

![[linear.svg]] ![[loglog.svg]]

Wykresy pokazują czas rozkładu LU w zależności od wielkości macierzy. W kolorze zielonym przedstawiona jest metoda ogólna, a na czerwono metoda zoptymalizowana dla macierzy pasmowych.

# Wyznacznik
Po rozkładzie LU, obliczenie wyznacznika również można obliczyć w czasie liniowym: $\det A = \prod\limits_{i=1}^N U_{i, i}$.

Dla macierzy z zadania numerycznego dla $N = 124$ wyznacznik jest równy $6141973498.857843399047852$.

# Rozwiązanie równania
Używając rozkładu LU, równanie $y = A^{-1}x$ rozwiązuje się używając wyłącznie forward i back substitution: najpierw $y_m = \frac{b_m - \sum_{i=1}^{m-1} l_{m,i} y_i}{l_{m,m}}$ dla $m = 1, ..., n$, a później $x_m = \frac{y_m - \sum_{i=m+1}^{n} u_{m,i} x_i}{u_{m,m}}$ for $m = n, ..., 1$. Tutaj również można użyć struktury macierzy, otrzymując zoptymalizowane równania: $y_m = b_m - \sum_{i=\max(1, m - L)}^{m-1} l_{m,i} y_i$ dla $m = 1, ..., n$ i $x_m = \frac{y_m - \sum_{i=m+1}^{\min(n, m + U)} u_{m,i} x_i}{u_{m,m}}$ dla $m = n, ..., 1$. Otrzymany wynik przykładowy (prawdziwy dla macierzy z zadania jest bardzo duży; w całości wypisuje go program) dla macierzy o wymiarach $10 \times 10$ to $y_{10} = \begin{bmatrix} 0.4487008278590469 \\ 1.4132732873429976 \\ 2.1348778522322926 \\ 2.869013253466097 \\ 3.5914886842267686 \\ 4.311606217445992 \\ 5.029800647623075 \\ 5.746749942177135 \\ 6.475040195123446 \\ 7.254159967479426 \end{bmatrix}$.
