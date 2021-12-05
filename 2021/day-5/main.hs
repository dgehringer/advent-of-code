import Data.List (group, sort)
import Data.List.Split (splitOn)

type Point = (Int, Int)
type Line = (Point, Point)

toTuple [a, b] = (a,b)
parseLines = map (toTuple . (map (toTuple . (map (read :: String -> Int) . splitOn ",") ). splitOn " -> "))
sign a = if a == 0 then 0 else a `div` abs a
isVertical ((x1, _), (x2, _)) = abs (x1 - x2) == 0
isHorizontal ((_, y1), (_, y2)) = abs (y1 - y2) == 0
add (x1, y1) (x2, y2) = (x1 + x2, y1 + y2)
sub (x1, y1) (x2, y2) = (x1 - x2, y1 - y2)
times n (x, y) = (n * x, n * y)

unitVector (p1, p2) = case p2 `sub` p1 of
    (dx, 0) -> ((sign dx, 0), abs dx)
    (0, dy) -> ((0, sign dy), abs dy)
    (a, b) -> if abs a == abs b then ((sign a, sign b), abs a) else error $ "A line with a slop of " ++ show (a, b) ++ " cannot be displayed"

pointsAlongLine l@(p1 , p2) = add p1 <$> (flip times u <$> [0..n])
    where (u , n) = unitVector l

constructAllPoints :: [Line] -> [Point]
constructAllPoints = foldl ((. pointsAlongLine) . (++)) []

countPointsWithMoreOverlapsThan n = length . filter (>= n) . map length . group . sort

problem1 = do
    lines <- filter (\l -> isVertical l || isHorizontal l) . parseLines . lines <$> readFile "input.txt"
    print $ (countPointsWithMoreOverlapsThan 2 . constructAllPoints) lines

problem2 = do
    lines <- parseLines . lines <$> readFile "input.txt"
    let allPoints = foldl (\lst ln -> lst ++ pointsAlongLine ln) [] lines
    print $ (countPointsWithMoreOverlapsThan 2 . constructAllPoints) lines
