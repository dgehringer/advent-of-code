{-# LANGUAGE TupleSections #-}

import Data.List (transpose)
import Control.Monad (liftM2)
import Data.List.Split (splitOn, chunksOf)

type Board = [[(Int, Bool)]]

parseBoards = chunksOf 5 . map (map ((, False) . (read :: String -> Int)) . (filter ([] /=) . splitOn " ")) . filter ([] /=)

readBoards s = (drawn, (parseBoards .tail) l)
    where l = lines s
          drawn = (map (read :: String -> Int) . splitOn ",") (head l)


markBoard num = map (map (\(n,s) -> (n, n == num || s) ))

isBingo = liftM2 (||) isRowBingo (isRowBingo . transpose)
    where isRowBingo = any (all snd)

playBingo b@(bds, c) n = if any isBingo bds then b else (map (markBoard n) bds, n)

computeScore bd lastn = lastn * sumUnmarked bd 
    where sumUnmarked = sum . map (sum . map fst . filter (not . snd))

problem1 = do
    (numbers, boards) <- readBoards <$> readFile "input.txt"
    let (final_boards, last_num) = foldl playBingo (boards, -1) numbers
        winner_board = (head . filter isBingo) final_boards 
    print $ computeScore winner_board last_num

playBingo' b@(bds, c) n = if length bds == 1 then playBingo b n else (filter (not . isBingo) (map (markBoard n) bds), n)

problem2 = do
    (numbers, boards) <- readBoards <$> readFile "input.txt"
    let ([last_board], last_num) = foldl playBingo' (boards, -1) numbers
    print $ computeScore last_board last_num
