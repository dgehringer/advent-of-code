import qualified Data.Map as M
import Data.List (sort, group)
import Data.List.Split (splitOn)

applyN n f = foldl (.) id (replicate n f)

makeFishHist f = M.fromList $ map (\idx -> if M.member idx currHist then (idx, currHist M.! idx) else (idx, 0)) [0..8] -- converts the list of fishes to a histogram map
    where currHist =  (M.fromList . map (\l -> (head l, length l)) . group . sort) f -- create a histogramm of the passed fishes

readFishesFromFile f = makeFishHist . map (read :: String -> Int) . splitOn "," . head . lines <$> readFile f

advanceStep fish = (M.adjust (+reproducingFishes) 6 . M.adjust (const reproducingFishes) 8) fish' -- add reproducing fishes to six and set the value to key 8
    where
        reproducingFishes =  fish M.! 0
        fish' = foldl (\m idx -> M.adjust (const (m M.! idx)) (idx -1) m) fish [1..8] -- degrade of lanternfishes /= 0

countFishes = sum . map snd . M.assocs

problem1 = do
    fish <- readFishesFromFile "input.txt"
    print $ countFishes (applyN 80 advanceStep fish)

problem2 = do
    fish <- readFishesFromFile "input.txt"
    print $ countFishes (applyN 256 advanceStep fish)
