Problem polega na rozwiązaniu równania $Ax = b$ metodą Jacobiego i Gaussa-Seidela i porównaniu tych metod.

# Równanie
Macierz $A = \begin{bmatrix}3 &1 &0.15 &0 &... &0 \\1 &3 &1 &0.15 &... &0 \\0.15 &1 &3 &1 &... &0 \\... &... &... &... &... &... \\0 &... &0 &0.15 &1 &3\end{bmatrix}$ oraz wektor $b = \begin{bmatrix}1 \\2 \\3 \\... \\N\end{bmatrix}$ są znane, i należy znaleść wartość wektora $x$. Dokładne rozwiązanie oraz kolejne iteracje obu metod przedstawia program (dla $N = 10$).

# Jacobi i Gauss-Seidel
Rozwiązanie równania $Ax = b$ można przybliżyć używając iteracyjnej metody Jacobiego:

$$
x_i^{(k+1)} = \frac{1}{a_{i,i}} (b_i - \sum_{j \ne i} a_{i,j}x_j^{(k)})
$$

Podobnie działa również metoda Gaussa-Seidela:

$$
x_i^{(k+1)} = \frac{1}{a_{i,i}} (b_i - \sum_{j = 1}^{i - 1} a_{i,j}x_j^{(k + 1)} - \sum_{j = i + 1}^n a_{i,j}x_j^{(k)})
$$

W obu metodach wektor początkowy $x^{(0)}$ może być dowolny. Rozwiązania, których błędy są pokazane na wykresach używały losowo wygenerowane wektory (gdzie każdy element $x^{(0)}_i$ jest jednomiernie losowany z przedziału $[0, 1)$).

![[jacobi.svg]] ![[gauss-seidel.svg]]

Wykresy przedstawiają zależność między błędem (norma różnicy $x^{(k)}$ i dokładnego rozwiązania) i iteracją ($k$). Widać, że w obu metodach rozwiązanie zbliża się do dokładnego, ale w przypadku metody Gaussa-Seidela proces ten potrzebuje mniej iteracji. Oba wykresy pokazują dane dla dziesięciu rozwiązań równania, ale dla wszystkich wektorów rozwiązanie daną metodą zajmuje podobną ilość iteracji.
