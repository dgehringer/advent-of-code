import scala.io.Source
import scala.util.Using
import scala.collection.mutable

type Pos   = (Int, Int)
type Dir   = Pos
type Entry = (Pos, Dir)
type Field = Array[Array[Byte]]

extension (p: Pos)
  def +(q: Pos): Pos = (p._1 + q._1, p._2 + q._2)
  def opposite: Pos  = (-p._1, -p._2)

extension (f: Field)
  def height: Int      = f.length
  def width: Int       = f.head.length
  def end: Pos         = (height - 1, width - 1)
  def at(p: Pos): Byte = f(p._1)(p._2)
  def inBounds(p: Pos): Boolean = p._1 >= 0 && p._2 >= 0 && p._1 < height && p._2 < width

val directions: Set[Pos] = Set((1, 0), (-1, 0), (0, 1), (0, -1))

def neighborsAlongDirection(f: Field, minStreak: Int, maxStreak: Int)(heat: Int, pos: Pos, dir: Dir): Iterable[(Int, Pos, Dir)] =
  (1 to maxStreak)
    .scanLeft((heat, pos, 0))((state, i) =>
      val (h, p, _) = state
      val npos      = p + dir // accumulate heat and pos along direction
      val heatAcc   = h + (if f inBounds npos then f at npos else 0)
      (heatAcc, npos, i),
    )
    .filter((_, _, i) => i >= minStreak,) // sort out those steps below min streak
    .filter((_, p, _) => f inBounds p)
    .map((h, p, _) => (h, p, dir))

def minimumHeatLossPath(f: Field, start: Pos, end: Pos, minStreak: Int, maxStreak: Int): Int =
  val findNeighbors = neighborsAlongDirection(f, minStreak, maxStreak)
  val q    = mutable.PriorityQueue((0, start, (0, 0)))(using Ordering.by(-_._1))
  val seen = mutable.HashSet.empty[Entry]
  while q.nonEmpty do
    val (heat, pos, dir) = q.dequeue()
    if pos == end then return heat
    else if !seen.contains((pos, dir)) then
      seen.add((pos, dir))
      val remainingDirections =
        directions - dir - dir.opposite // exclude the current and its opposite direction
      q.addAll(remainingDirections.flatMap(findNeighbors(heat, pos, _)))
  -1

@main
def main(): Unit =
  val blocks =
    Using(Source.fromFile("input.txt"))(src =>
      src.getLines().map(_.map(_.toString.toByte).toArray).toArray,
    ).get
  val partOne = minimumHeatLossPath(blocks, (0, 0), blocks.end, 1, 3)
  val partTwo = minimumHeatLossPath(blocks, (0, 0), blocks.end, 4, 10)
  println(s"Part one: $partOne, Part two: $partTwo")
