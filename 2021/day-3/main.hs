import Data.List (transpose, isPrefixOf)

binToDec :: String -> Int
binToDec = foldl (\a d -> a*2 + if d == '1' then 1 else 0) 0

countBits comp b = if balance b `comp` 0 then '1' else '0'
    where balance = sum . map (\c -> if c == '1' then 1 else -1)

problem1 = do
    inp <- lines <$> readFile "input.txt"
    let gamma = binToDec (countBits (>=) <$> transpose inp)
    let epsilon = binToDec (countBits (<=) <$> transpose inp)
    print $ gamma * epsilon

computeRate pred numbers pref = 
    case numbers of
        [a] -> a
        _ -> computeRate pred (filter (isPrefixOf newPrefix) numbers) newPrefix
    where nextBit = pred . (!! length pref) . transpose
          newPrefix = pref ++ [nextBit numbers]

problem2 = do
    inp <- lines <$> readFile "input.txt"
    let oxygen_rate = binToDec $ computeRate (countBits (>=)) inp ""
    let scrubber_rate = binToDec $ computeRate (countBits (<)) inp ""
    print $ oxygen_rate * scrubber_rate
