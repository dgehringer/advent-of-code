import Data.List (nub)
import Control.Monad (liftM2)
import Data.List.Split (splitOn)
import Data.Char (isLower, isUpper)
import qualified Data.MultiSet as S
import qualified Data.Map as M

readEdges = map ((\[a, b] -> (a,b))  . splitOn "-") . lines

other n (a, b) = if a == n then b else a

isSmall = all isLower

parseCaveSystem s = (M.fromList . map (\c -> (c, findNeigbors c edges)) . distinctCaves) edges
    where
        edges = readEdges s
        findNeigbors n = S.fromList . map (other n) . filter (liftM2 (||) ((==n) . fst) ((==n) . snd))
        distinctCaves = nub . foldl (\l (a,b) -> a:b:l) []

searchCaves pred system = search "start" S.empty
    where
        search "end" _ = 1
        search cave smallCaves
            | pred cave smallCaves = 0
            | otherwise = (sum . map (`search` smallCave') . S.toList)  (system M.! cave)
            where smallCave' = if isSmall cave then cave `S.insert` smallCaves else smallCaves

canVisit cave smalls = (seen == 2) || (seen == 1 && (cave == "start" || sawOneTwice smalls))
    where sawOneTwice = any ((>1) . snd) . S.toOccurList
          seen = cave `S.occur` smalls

problem1and2 = do
    caveSystem <- parseCaveSystem <$> readFile "input.txt"
    print $ searchCaves S.member caveSystem
    print $ searchCaves canVisit caveSystem