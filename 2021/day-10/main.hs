import Data.List (sort)
import qualified Data.Map as M

data ChunkState =  Corrupted Char | Incomplete String deriving Eq

isCorrupted (Corrupted _) = True
isCorrupted _ = False

brackets = M.fromList [('(', ')'), ('[',']'), ('{', '}'), ('<', '>')]
closingBrackets = (map snd . M.assocs) brackets

analyseChunk :: String  -> ChunkState
analyseChunk = foldl matchBrackets (Incomplete [])
    where
        matchBrackets (Corrupted a) _ = Corrupted a
        matchBrackets (Incomplete []) a = Incomplete [a]
        matchBrackets (Incomplete r@(x:xs)) ch
            | M.member ch brackets = Incomplete (ch:r)
            | ch `elem` closingBrackets = if ch == (brackets M.! x) then Incomplete xs else Corrupted ch
            | otherwise = error "Illeagal character"

autoComplete (Incomplete s@(x:xs)) = map (brackets M.!) s
autoComplete _ = error "Can only autocomplete Incomplete chunks"

problem1 = do
    chunks <- lines <$> readFile "input.txt"
    let corrupted = (filter isCorrupted . map analyseChunk) chunks
    print $ (sum . map ((scoreBoard M.!) . (\(Corrupted c) -> c))) corrupted
    where scoreBoard = M.fromList [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]

problem2 = do
    chunks <- lines <$> readFile "input.txt"
    let incomplete = (filter (not . isCorrupted) . map analyseChunk) chunks
    print $ ((\l -> l !! (length l `div` 2)) . sort . map (score . autoComplete)) incomplete
    where 
        score = foldl (\sc br -> 5*sc + (scoreBoard M.! br)) 0
            where scoreBoard = M.fromList [(')', 1), (']', 2), ('}', 3), ('>', 4)]
