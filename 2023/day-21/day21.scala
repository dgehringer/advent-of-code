import scala.collection.mutable
import scala.io.Source
import scala.math.floor
import scala.util.Using

type Pos   = (Int, Int)
type Field = Array[String]

extension (p: Pos) def +(q: Pos): Pos = (p._1 + q._1, p._2 + q._2)

extension (a: Int)
  def mod(n: Int): Int = a - (n * math.floor(a.toDouble / n).toInt)

extension (f: Field)
  def at(p: Pos): Char = f(p._1)(p._2)
  def h: Int           = f.length
  def w: Int           = f.head.length
  def inBounds(p: Pos): Boolean =
    p._1 >= 0 && p._1 < h && p._2 >= 0 && p._2 < w

def parseInput(lines: Iterator[String]): (Field, Pos) =
  val array = lines.toArray
  val y     = array.indexWhere(_ contains 'S')
  val x     = array(y) indexOf 'S'
  array.update(y, array(y).replace('S', '.'))
  (array, (y, x))

val directions: Seq[Pos] = Seq((1, 0), (-1, 0), (0, 1), (0, -1))

def countReachableTiles(field: Field, pos: Pos, n: Int): Int =
  var currentStep = Set(pos)
  for _ <- 0 until n do
    currentStep = currentStep flatMap (p =>
      directions
        .map(_ + p)
        .filter(p => field.inBounds(p) && field.at(p) != '#')
    )
  currentStep.size

def reachableTilesPeriodic(field: Field, pos: Pos, n: Int): BigInt =
  var currentStep = Set(pos)
  val seenRegions = mutable.ListBuffer.empty[BigInt]
  for size <- 0 until 3 * field.h do
    if size % field.h == field.h / 2 then
      println(s"\t$size ${currentStep.size}")
      seenRegions.prepend(currentStep.size)
    currentStep = currentStep.flatMap(pos =>
      directions
        .map(d => pos + d)
        .filter(p => field.at((p._1 mod field.h, p._2 mod field.w)) == '.'),
    )
  assert(seenRegions.size == 3)
  val (left, center, right) = (seenRegions(2), seenRegions(1), seenRegions.head)
  val nn                    = BigInt(n) / field.h
  left + nn * (center - left + (nn - 1) * (right - 2 * center + left) / 2)

@main
def main(): Unit =
  val (field, start) =
    Using(Source.fromFile("input.txt"))(src => parseInput(src.getLines())).get
  val partOne = countReachableTiles(field, start, 64)
  val partTwo = reachableTilesPeriodic(field, start, 26501365)
  println(s"Part one: $partOne, Part two: $partTwo")
