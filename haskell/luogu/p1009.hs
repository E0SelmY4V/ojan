main = do
    num <- getLine
    print . factSum . read $ num

factSum :: Integer -> Integer
factSum n = calc n
    where
        calc 1 = 1
        calc i = (n - i + 2) * calc (i - 1) + 1
