

readInt :: String -> Int
readInt = read

readIntList :: String -> IO [Int]
readIntList fname = do fmap (map readInt . lines) (readFile fname)

windowed :: Int -> [a] -> [[a]]
windowed s l =
    case l of
        [] -> []
        x:xs -> if length l >= s then
                    take s l : windowed s xs
                else windowed s xs

numIncreasingMeasurements :: Int ->[Int] -> Int
numIncreasingMeasurements ws numbers = (sum . map fromEnum) (zipWith (<) (init average) (tail average))
    where average = map sum (windowed ws numbers)

problem1 = do fmap (numIncreasingMeasurements 1) (readIntList "input.txt")

problem2 = do fmap (numIncreasingMeasurements 3) (readIntList "input.txt")