import Text.Parsec
import Control.Monad (liftM2)
import qualified Data.Set as S
import qualified Data.Matrix as M

type Mark = (Int, Int)
type Fold = (Char, Int)
type Paper = S.Set Mark

mirror axis c
    | c < axis = c
    | otherwise = axis - (c -axis)

foldPaper ('y', axis ) = S.map (liftM2 (,) fst (mirror axis  . snd))
foldPaper ('x', axis ) = S.map (liftM2 (,) (mirror axis  . fst) snd)
foldPaper (_ , _) = error "Can only fold along x and y direction"

showPaper paper = (M.transpose . S.foldl (\m (i, j) -> M.setElem '#' (i+1, j+1) m) emptyPaper) paper
    where (n, m) = S.foldl (\(a, b) (c, d) -> (max a c, max b d)) (0,0) paper
          emptyPaper = M.matrix (n+1) (m+1) (const '.')

problem1and2 = do
    content <- readFile "input.txt"
    let (Right (paper, folds)) = parse parserManual "" content
    print $ (S.size . foldPaper (head folds)) paper
    let foldedPaper = foldl (flip foldPaper) paper folds
    print $ showPaper foldedPaper

parserManual :: Parsec String () (Paper, [Fold])
parserManual = do
    coords <- parserMark `sepEndBy1` newline
    newline
    folds <- parserFold `sepEndBy1` newline
    return (S.fromList coords, folds)

parserMark :: Parsec String () Mark
parserMark = do
    [i,j] <- number `sepBy1` char ','
    return (i, j)

parserFold :: Parsec String () Fold
parserFold = do
    string "fold along" >> space
    direction <- char 'x' <|> char 'y'
    char '='
    axis <- number
    return (direction, axis)

number :: Parsec String () Int
number = (read :: String -> Int) <$> many1 digit