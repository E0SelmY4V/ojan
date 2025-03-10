main = do
    line <- getLine
    let (n, _ : x : _) : _ = reads line
    print $ countNum n x

countNum :: Int -> Char -> Int
countNum n x = sum . map (length . filter (== x) . show) $ [1..n]
