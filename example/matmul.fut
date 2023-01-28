let dotprod [n] (xs: [n]i32) (ys: [n]i32): i32 =
    i32.(reduce (+) (i32 0) (map2 (*) xs ys))

entry matmul [n][p][m] (xss: [n][p]i32) (yss: [p][m]i32): [n][m]i32 =
    map (\xs -> map (dotprod xs) (transpose yss)) xss
