Dla zadanego zbioru punktów wygenerowanych z zaburzeniami za pomocą funkcji $f(x, \beta) = \beta_1 * x^2 + \beta_2 * \sin(x) + \beta_3 * \cos(5x) + \beta_4 * \exp(-x)$ należy znaleść warości parametrów $\beta$.

# Metoda Gaussa-Newtona
Aby obliczyć minimum błędu $R^2 = S(\beta) = \sum\limits_i r_i(\beta)^2$ dla $r_i(\beta) = y_i - f(x_i, \beta)$ dla podanych $x$ i $y$, należy znaleść wartość $\beta$ dla której gradient tej funkcji jest równy $0$. Można to zrobić iteracyjnie: $\beta^{(k + 1)} = \beta^{(k)} - (J^TJ)^{-1} J^T r(\beta^{(k)})$, gdzie $r$ to wektor wszystkich $r_i$, a $J$ to macierz Jacobiana zdefiniowana jako $J_{i,j} = \frac{\delta r_i(\beta^{(k)})}{\delta \beta_j}$. Przekształcając ten wzór (z $\Delta = \beta^{(k+1)} - \beta^{(k)}$) można otrzymać równanie $(J^TJ)\Delta = -J^Tr(\beta^{(k)})$.

# Wyniki dla funkcji z zadania
Dla funkcji z zadania przybliżone parametry funkcji mają wartości $\beta \approx \begin{bmatrix} 0.101 &4.023 &3.089 &5.633 \end{bmatrix}^T$.

![[a 2.svg]]

Na wykresie podane punkty są zaznaczone na czerwono, a przybliżona funkcji na niebiesko.

# Wyniki dla innej funkcji
Podobna aproksymacja została przeprowadzona dla funkcji $g(x, \beta) = \beta_1 * x^2 + \beta_2 * x^3 + \beta_3 * \log(x) + \beta_4 * \sin(x)$ z losowo wygenerowanymi parametrami $\beta$.

Przybliżone parametry funkcji dla różnych losowych parametrów $\beta$ przy kilku wykonaniach programu każdym razem były bardzo bliskie, ale nie dokładnie równe, początkowym parametrom. Na wykresie jest pokazany przykład dla $\beta \approx \begin{bmatrix} 0.27285 &0.35048 &2.3246 &22.481 \end{bmatrix}^T$, gdzie przybliżone parametry miały wartości $\tilde\beta \approx \begin{bmatrix} 0.26335 &0.35109 &2.4068 &22.572 \end{bmatrix}^T$.

![[b 2.svg]]

Na wykresie wygenerowane punkty są zaznaczone na czerwono, początkowa funkcja przerywaną linią na zielono, a przybliżona funkcji na niebiesko. Widać, że przybliżona funkcja prawie całkowicie pokrywa początkową.
