
import Data.List (splitAt, nub, maximum, minimum)
import Data.List.Split (splitOn)
import qualified Data.Map as M
import qualified Data.MultiSet as S

type Polymer = S.MultiSet String
type Rules = M.Map String Char

parseRules s = (head lns, (M.fromList . map (parseRule . splitOn " -> ")) rules')
    where lns = lines s
          (_, rules') = splitAt 2 lns
          parseRule [[a,b], [c]] = (a:[b],c)
          parseRule _ = error "Rule must be of type \"xx\" -> 'x'"

windowed :: Int -> [a] -> [[a]]
windowed s l =
    case l of
        [] -> []
        x:xs -> if length l >= s then
                    take s l : windowed s xs
                else windowed s xs

polymerize p r = foldl (\poly (a, occur) -> S.insertMany a occur poly) S.empty (newPairs p)
    where transformPair (l@[a,b], occ) = if l `M.member` r then [(a:[c], occ), (c:[b], occ)] else [(l,occ)]
               where c = r M.! l
          transformPair _ = error "Invalud transform"
          newPairs = concatMap transformPair . S.toOccurList

score p l = (maximum hist' `div` 2) - (minimum  hist' `div` 2) + 1
    where occurrences = (concatMap (\([a,b], occ) -> [(a, occ), (b, occ)]) . S.toOccurList) p
          emptyHist = M.fromList $ zip ((nub . map fst) occurrences) (repeat 0)
          hist = foldl (\m (c, occ) -> M.adjust (+occ) c m) emptyHist occurrences
          hist' = (M.elems . M.adjust (+1) l) hist

polymerizeN n p r = foldl (\pp _ -> polymerize pp r) p [1..n]

polymerLength = (+ 1) . sum . map snd . S.toOccurList

main = do
    (polymer', rules) <- parseRules <$> readFile "input.txt"
    let polymer = (S.fromList . windowed 2) polymer'
        p1 = polymerizeN 10 polymer rules
        p2 = polymerizeN 40 polymer rules
        l = last polymer'
    print $ score p1 l
    print $ score p2 l
