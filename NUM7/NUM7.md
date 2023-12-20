Dla podanej funkcji $f(x) = \frac{1}{1 + 50x^2}$ należy znaleść wielomiany interpolacyjne.

> [![[plot.webp]]](https://raw.githubusercontent.com/j-markiewicz/mn/main/NUM7/plot.webp)
> *Jeśli nie wyświetla się animacja można otworzyć ją w przegłądarce klikając na wykres.*

# Rozkład jednomierny
Jeżeli wielomian interpolacyjny zostanie wygenerowany używając jednomiernie rozmieszczonych węzłów interpolacji ($x_i = -1 + 2\frac{i}{n+1}$ dla $i = 0, 1, ..., n$), to nawet dla małych $n$ część funkcji $f$ blisko $x = 0$ jest dobrze zinterpolowana. Ta część wykresu polepsza się z rosnącym $n$. Bardzo inaczej jest jednak dla części wykresu $f$ bliżej $x = \pm1$, gdzie pojawiąją się bardzo duże oscylacje (Efekt Rungego).

![[a 1.svg]]

Na wykresie są pokazane wartości wielomianu interpolacyjnego dla $n = 3, 5, 10, 30$ oraz dokładna wartość funkcji (linią przerywaną). Widać, że pojawiają się oscylacje po bokach wykresu, które dla większych $n$ stają się bardzo duże. Widać również, że blisko $x = 0$ z rosnącym $n$ wykres szybko się przybliża do wykresu funkcji $f$.

# Rozkład Czebyszewa
Jeśli wielomian interpolacyjny zostanie wygenerowany dla węzłów rozmieszczonych niejednomierne, biorąc składową $x$ punktów rozmieszczonych jednomiernie na okręgu ($x_i = \cos(\pi \frac{2i + 1}{2(n + 1)})$), to problem oscylacji z poprzedniej części znika. Natomiast aby dokładniej zinterpolować wartości blisko $x = 0$ trzeba zwiększyć $n$.

![[b 1.svg]]

Przedstawione są podobne wykresy jak poprzednio (dla $n = 3, 5, 10, 30$). Widać, że dla $n < 10$, wielomian interpoluje funkcję $f$ bardzo niedokładnie blisko $x = 0$. Dla więszych $n$ interpolacja się polepsza, i nie powstają duże oscylacje jak poprzednio, a wykres interpolacji dla $n = 30$ prawie całowicie się pokrywa z wykresem funkcji $f$.
