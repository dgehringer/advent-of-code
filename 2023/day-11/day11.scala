
import scala.io.Source
import scala.util.Using

type Pos = (Int, Int)
type Universe = Array[Array[Char]]
type Folds = (List[Int], List[Int]) // multiplicities col indices, row indices

def findGalaxies(universe: Universe, multiplicity: Int): Array[Pos] =
  val folds = findFolds(universe)
  val unfoldPos = unfoldPoint(folds)(multiplicity)
  universe.zipWithIndex
    .flatMap((line, y) =>
      line.zipWithIndex
        .filter(p => p._1 == '#')
        .map((_, x) => unfoldPos(x, y))
    )

def findFolds(g: Universe): Folds =
  def foldsAlongAxis(galaxy: Universe): List[Int] =
    galaxy.zipWithIndex.filter(_._1.forall(_ == '.')).map(_._2).toList

  (foldsAlongAxis(g.transpose), foldsAlongAxis(g)) // multiplicities col indices, row indices

def unfoldPoint(folds: Folds)(multiplicity: Int)(p: Pos): Pos =
  (p._1 + (multiplicity - 1) * folds._1.count(_ < p._1), p._2 + (multiplicity - 1) * folds._2.count(_ < p._2))

def galaxyPairDistance(galaxies: Array[Pos]): Long =
  (for
    i <- galaxies.indices
    j <- i + 1 until galaxies.length
  yield shortestPathLength(galaxies(i), galaxies(j))).sum

def shortestPathLength(a: Pos, b: Pos): Long =
  val (dx, dy) = ((b._1 - a._1).abs, (b._2 - a._2).abs)
  if dx < dy then dx * 2 + (dy - dx) else dy * 2 + (dx - dy)

@main
def main(): Unit =
  val contractedUniverse = Using(Source.fromFile("input.txt"))(_.getLines().map(_.toArray).toArray).get
  val partOne = galaxyPairDistance(findGalaxies(contractedUniverse, 2))
  val partTwo = galaxyPairDistance(findGalaxies(contractedUniverse, 1000000))
  println(s"Part one: $partOne, Part two: $partTwo")
