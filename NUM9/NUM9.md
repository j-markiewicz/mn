Problem polega na numerycznym wyznaczeniu miejsca zerowego funkcji $f(x) = sin(x) - 0.4$ oraz $g(x) = f^2(x)$. Do rozwiązania tego problemu można zastosować kilka metod:

- Metodę bisekcji\* *(tylko do funkcji $f$)*
- Metodę falsi\* *(tylko do funkcji $f$)*
- Metodę siecznych\*\*
- Metodę Newtona
- "Polepszoną" metodę Newtona używając funkcji $u(x) = \frac{g(x)}{g'(x)}$ *(tylko dla funkcji $g$)*

\* Jako zbliżona wartość miejsca zerowego $x^*$ w danej iteracji jest brany środek przedziału.
\*\* Jako zbliżona wartość miejsca zerowego $x^*$ w danej iteracji jest brane miejsce zerowe siecznej.

## Funkcja $f$

![[f.svg]]

Widać, że dla funkcji $f(x) = sin(x) - 0.4$ metoda Newtona zbiega najszybciej, potrzebując tylko czterech iteracji do osiągnięcia błędu mniejszego niż $10^{-15}$. Metoda siecznych jest trochę wolniejsza - potrzebuje siedem iteracji. Metoda falsi przez kilkanaście pierwszych iteracji wydaje się zbliżać do nieprawidłowej wartości miejsca zerowego. Ten efekt jest spowodowany przez powyższą definicję miejsca zerowego dla tej metody (\*). Dla funkcji $f$ metoda zmniejsza przedział początkowo tylko z jednej strony, zbliżając koniec przedziału do $x^*$ ale pozostawiając początek przedziału w $0$ (aż do ostatniej iteracji). Metoda bisekcji dość powoli zbliża się do dokładnej wartości; potrzebuje około 50 iteracji do otrzymania błędu mniejszego niż $10^{-15}$.

## Funkcja $g$

![[g.svg]]

Dla funkcji $g(x) = (sin(x) - 0.4)^2$ już nie odpowiadają metody bisekcji i falsi, ponieważ nie są osiągnięte wartości ujemne. Widać również efekty różnicy pomiędzy warunkiem zaprzestania iteracji ($g(x^*_i) < 10^{-15}$) a wykreślonym błędem ($\epsilon_i = |\arcsin 0.4 - x^*_i|$, gdzie $\arcsin 0.4 \approx 0.4115$ jest dokładnym miejscem zerowym). Metoda Newtona i polepszona Newtona osiągają wartość błędu mniejsze niż tylko $10^{-7} \approx \sqrt{10^{-15}}$. Metoda siecznych zbliża się do miejsca zerowego w $\pi - \arcsin 0.4$, co powoduje wysoki błąd w porównaniu do miejsca zerowego w $\arcsin 0.4$. Błąd między miejscem zerowym otrzymanym przez metodę siecznych po 36 iteracjach a $\pi - \arcsin 0.4$ wynosi około $2.2 * 10^{-8}$.
Dla funkcji $g$, metoda "polepszona" Newtona jest najszybsza, zbiegając po trzech iteracjach. Metoda Newtona nieużywająca funkcji $u$ zbiega po 24 iteracjach, a metoda siecznych po 36. Przy wyborze metody należy jednak uwzględnić, że "polepszona" metoda Newtona wymaga wyliczenia pierwszej **i** drugiej pochodnej funkcji $g$, a także więcej innych operacji matematycznych.
