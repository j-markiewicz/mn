Zadana jest macierz $M = \begin{bmatrix} 8 &1 &0 &0 \\1 &7 &2 &0 \\0 &2 &6 &3 \\0 &0 &3 &5 \end{bmatrix}$, i dla niej należy znaleść wartości i wektory własne.

# Metoda potęgowa
Metoda potęgowa polega na obliczeniu kolejnych iteracji $\tilde y_k = Ay_{k-1}$, $y_k = \frac{\tilde y_k}{||\tilde y_k||}$. Ta metoda dla macierzy rzeczywistych symetrycznych dla (praktycznie) dowolnego wektora początkowego $y_1$ zbiega do wektora własnego odpowiadającego bezwzględnie największej wartości własnej.

![[a.svg]]

Na wykresie przedstawiona jest zmiana wektora ($||y_{k-1} - y_k||$) w kolejnej iteracji od iteracji ($k$) dla pięciu losowych wektorów początkowych. Widać, że metoda potęgowa zbiega do bardzo małego błędu po kilkudziesięciu iteracjach.

# Algorytm QR
Wartości własne macierzy można również obliczyć algorytmem QR polegającym na rozkładzie QR macierzy $A_i = Q_iR_i$. Kolejne iteracje mnożą $A_{i+1} = R_iQ_i$ i ponownie rozkładają macierz.

![[b.svg]]

Wykres pokazuje (ciągłe linie, skala po lewej) wartości na diagonali macierzy $A_{i_{j,j}}$ zależnie od iteracji $i$. Widać, że po kilkunastu iteracjach zbiegają one do pewnych wartości. Te wartości są wartościami własnymi tej macierzy. Na wykresie również widać (przerywane linie, skala po prawej) wartości macierzy pod główną diagonalą. Trzy z nich są równe $0$ od początku, a pozostałe zbiegają do zera, co pokazuje, że macierz w kolejnych iteracjach upodabnia się do macierzy trójkątnej górnej.
