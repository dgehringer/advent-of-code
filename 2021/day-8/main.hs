import qualified Data.Map as M
import qualified Data.Set as S
import Data.List.Split (splitOn)

parseSignals = map ((\[a, b] -> (a,b)) . map (splitOn " ") . splitOn " | ") . lines

count1478 = length . filter (\s -> length s `elem` [2, 3, 4, 7]) . concatMap snd

problem1 = do
    signals <-  parseSignals <$> readFile "input.txt"
    print $ count1478 signals

findFirst pred = head . filter pred

deduceMapping line = M.fromList $ zip numbers ['0'..'9']
    where l = (map S.fromList . fst) line
          ofLength n =  filter ((==n) . S.size) l
          ofLength' n = head (ofLength n)
          numOne = ofLength' 2
          numFour = ofLength' 4
          numSeven = ofLength' 3
          numEight = ofLength' 7
          findFirst' = flip findFirst (ofLength 6) -- find among candidate 0 6 9
          numNine = findFirst' (numFour `S.isSubsetOf`)
          numZero = findFirst' (\s -> s /=numNine && S.intersection s numSeven == numSeven)
          numSix = findFirst' (\s -> s /= numNine && s/= numZero)
          findFirst'' = flip findFirst (ofLength 5)
          numTwo = findFirst'' (not . (`S.isSubsetOf` numNine))
          numThree = findFirst'' (\s -> s /=numTwo && S.intersection s numSeven == numSeven)
          numFive = findFirst'' (\s -> s /= numTwo && s/= numThree)
          numbers = [numZero, numOne, numTwo, numThree, numFour, numFive, numSix, numSeven, numEight, numNine]

computeNumber mapping = (read :: String -> Int) .map ((M.!) mapping . S.fromList) . snd

problem2 = do
    signals <- parseSignals <$> readFile "input.txt"
    print $ (sum . map (computeNumber =<< deduceMapping)) signals
