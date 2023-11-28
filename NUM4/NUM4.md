Problem polega na rozwiązaniu równania $Ax = b$ dla macierzy $A$ o specyficznej strukturze - $A$ ma strukturę przypominającą macierz pasmową (z jednym pasmem nad główną diagonalą), ale zamiast $0$ na pozostałych pozycjach, $A$ ma $1$. W ogólnym przypadku rozwiązanie równania miałoby złożoność obliczeniową $O(N^3)$. Wykorzystując strukturę macierzy, można rozwiązać to równanie w czasie $O(N)$.

# Macierz $A$
Macierz $A = \begin{bmatrix}12 &8 &1 &1 &... &1 \\1 &12 &8 &1 &... &1 \\1 &1 &12 &8 &... &1 \\... &... &... &... &... &... \\1 &1 &1 &1 &... &12\end{bmatrix}$ można rozłożyć według wzoru $A = A' + uu^T$ na macierz pasmową $A' = \begin{bmatrix}11 &7 &0 &0 &... &0 \\0 &11 &7 &0 &... &0 \\0 &0 &11 &7 &... &0 \\... &... &... &... &... &... \\0 &0 &0 &0 &... &11\end{bmatrix}$ i wektor $u = \begin{bmatrix}1 \\1 \\1 \\... \\1\end{bmatrix}$.

# Sherman-Morrison
Dla macierzy $A$ można zastosować wzór Shermana-Morrisona $(A' + uv^T)^{-1} = A'^{-1} - \frac{A'^{-1}uv^TA'^{-1}}{1 + v^TA'^{-1}u}$ z $u = v$. Używając tego wzoru można rozwiązać równanie wykonując kolejne kroki:

1. Rozkład $A' = LU$
2. Rozwiązanie równania $LU\vec y = \vec b$
3. Rozwiązanie równania $LU\vec z = \vec u$
4. Obliczenie $\vec x = \vec y - \frac{\vec z\vec u^T\vec y}{1 + \vec u^T\vec z}$, gdzie:
   - $\vec z\vec u^T\vec y = \bigg(\sum\limits_{i=1}^N \vec y_i\bigg)\vec z$
   - $\vec u^T\vec z = \sum\limits_{i=1}^N \vec z_i$

# Rozwiązanie równania
Dzięki pasmowej strukturze $A'$, przy zastosowaniu powyższych wzorów można rozwiązać równanie w czasie liniowym:

![[linear 1.svg]] ![[loglog 1.svg]]

Wykresy pokazują czas rozwiązania równania w zależności od wielkości macierzy. Z pierwszego wykresu widać, że czas wykonania jest proporcjonalny do wielkości macierzy ($N$). Drugi wykres dodatkowo pokazuje, że obliczenia mają pewien stały koszt niezależny od wielkości macierzy w dodatku do wyżej wymienionego kosztu liniowego.

Otrzymany wynik przykładowy (prawdziwy dla macierzy z zadania jest bardzo duży, w całości wypisuje go program) dla $N = 10$ to $x_{10} \approx \begin{bmatrix} 0.17423 \\0.17917 \\0.17141 \\0.18360 \\0.16445 \\0.19453 \\0.14726 \\0.22155 \\0.10482 \\0.28825 \end{bmatrix}$.
