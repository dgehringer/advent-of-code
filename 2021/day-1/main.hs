
import Control.Monad (liftM)
import Distribution.ParseUtils (fName)

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

numIncreasingMesurements :: Int ->[Int] -> Int
numIncreasingMesurements ws numbers = (sum . map fromEnum) (zipWith (<) (init average) (tail average))
    where average = map sum (windowed ws numbers)

problem1 = do fmap (numIncreasingMesurements 1) (readIntList "input.txt")


problem2 = do fmap (numIncreasingMesurements 3) (readIntList "input.txt")