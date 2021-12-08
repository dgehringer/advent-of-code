import Data.List.Split (splitOn)

minCrabAlignment metric c = minimum (map (sum . (\pos -> map (metric . abs . (pos -)) c)) [1..(maximum c)])
gaussianSumFormula n = (n * (n-1)) `div` 2
    
problem1and2 = do
    crabs <-  map (read :: String -> Int) . splitOn "," . head . lines <$> readFile "input.txt"
    print $ minCrabAlignment id crabs
    print $ minCrabAlignment (gaussianSumFormula . (1+)) crabs