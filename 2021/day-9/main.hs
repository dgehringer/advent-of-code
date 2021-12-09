
import Data.List (sort)
import Data.Char (digitToInt)
import qualified Data.Set as S
import qualified Data.Matrix as M
import Data.Maybe (mapMaybe, isJust, fromJust)

readMatrix = M.fromLists . map (map digitToInt) . lines

safeGet' m (i, j) = M.safeGet i j m

adjacent mat (x, y) = mapMaybe (safeGet' mat) [(x-1, y), (x + 1, y), (x, y -1), (x, y + 1)]

adjacentWithIndex mat (x,y) = map (\p -> (p, safeGet' mat p)) [(x-1, y), (x + 1, y), (x, y -1), (x, y + 1)]

isFloor mat p = all ((mat M.! p)<) (adjacent mat p)

cartesianProduct a b = S.toList (S.cartesianProduct (S.fromList a) (S.fromList b))

coordList m = cartesianProduct [1..(M.nrows m)] [1..(M.ncols m)]

problem1 = do
    heightMap <- readMatrix <$> readFile "input.txt"
    print $ (sum . map ((1+) . (heightMap M.!)) . filter (isFloor heightMap)) (coordList heightMap)

basinSize mat p = S.size (basinSize' p (S.fromList [p]))
    where
        basinSize' p' basin
            | grownBasin == basin = grownBasin
            | otherwise = S.unions (S.map (`basinSize'` grownBasin) (S.difference grownBasin basin))
            -- filter those indices which are not 9 and merge them with the old basin
            where grownBasin = (S.union basin . S.fromList . map fst . filter (isFlowPoint .snd)) (adjacentWithIndex mat p')
                  currentHeight = mat M.! p'
                  isFlowPoint pp = isJust pp && pp /= Just 9 && fromJust pp > currentHeight

problem2 = do
    heightMap <- readMatrix <$> readFile "input.txt"
    let basins = (filter (isFloor heightMap) . coordList) heightMap
    print $ (product . take 3 . reverse . sort. map (basinSize heightMap)) basins
