A1 = {
	{2.554219275, 0.871733993, 0.052575899, 0.240740262, 0.316022841},
	{0.871733993, 0.553460938, -0.070921727, 0.255463951, 0.707334556},
	{0.052575899, -0.070921727, 3.409888776, 0.293510439, 0.847758171},
	{0.240740262, 0.255463951, 0.293510439, 1.108336850, -0.206925123},
	{0.316022841, 0.707334556, 0.847758171, -0.206925123, 2.374094162}
}

A2 = {
	{2.645152285, 0.544589368, 0.009976745, 0.327869824, 0.424193304},
	{0.544589368, 1.730410927, 0.082334875, -0.057997220, 0.318175706},
	{0.009976745, 0.082334875, 3.429845092, 0.252693077, 0.797083832},
	{0.327869824, -0.057997220, 0.252693077, 1.191822050, -0.103279098},
	{0.424193304, 0.318175706, 0.797083832, -0.103279098, 2.502769647}
}

b = {{-0.642912346}, {-1.408195475}, {4.595622394}, {-5.073473196}, {2.178020609}}
deltab = ResourceFunction["RandomUnitVector"][5] / 10^6

y1 = Quiet[LinearSolve[A1, b]] (* Quiet, aby nie był wypisany błąd o niedokładnych wynikach spowodowanych źle określoną macierzą *)
y2 = LinearSolve[A2, b]

y1zaburzone = Quiet[LinearSolve[A1, b + deltab]]
y2zaburzone = LinearSolve[A2, b + deltab]

Print["A_1 = ", A1]
Print["A_2 = ", A2]
Print["k(A_1) = ", Quiet[Norm[A1,2]Norm[Inverse@A1,2]]]
Print["k(A_2) = ", Quiet[Norm[A2,2]Norm[Inverse@A2,2]]]
Print["||A_1 - A_2||_2 = ", Norm[A1 - A2]]

Print[]

Print["y_1 = ", y1]
Print["y_2 = ", y2]
Print["||y_1 - y_2||_2 = ", Norm[y1 - y2]]
Print["||y_1 - y_2||_inf = ", Norm[y1 - y2, Infinity]]

Print[]

Print["deltab = ", deltab]
Print["||deltab||_2 = ", Norm[deltab]]

Print[]

Print["y_1zaburzone = ", y1zaburzone]
Print["y_2zaburzone = ", y2zaburzone]

Print[]

Print["||y_1zaburzone - y_2zaburzone||_2 = ", Norm[y1zaburzone - y2zaburzone]]
Print["||y_1 - y_1zaburzone||_2 = ", Norm[y1 - y1zaburzone]]
Print["||y_2 - y_2zaburzone||_2 = ", Norm[y2 - y2zaburzone]]
