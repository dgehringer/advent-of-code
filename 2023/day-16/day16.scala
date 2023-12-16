import scala.collection.mutable
import scala.io.Source
import scala.util.Using

type Pos   = (Int, Int)
type Field = Array[Array[Char]]

object Direction:
  val north: Pos = (-1, 0)
  val south: Pos = (1, 0)
  val east: Pos  = (0, 1)
  val west: Pos  = (0, -1)

def energizedTileCount(beam: Beam, field: Field): Int =
  val (h, w) = (field.length, field.head.length)

  @inline
  def inBounds(p: Pos) = p._1 >= 0 && p._1 < h && p._2 >= 0 && p._2 < w

  val remainingBeams = mutable.Stack(beam)
  val seenBeams      = mutable.Set[Beam]()
  while remainingBeams.nonEmpty do
    val beam = remainingBeams.pop()
    if inBounds(beam.pos) && !seenBeams.contains(beam) then
      seenBeams += beam
      remainingBeams.pushAll(beam.step(field))
  seenBeams.map(_.pos).size

def maxEnergizedTileCount(field: Field): Int =
  import Direction.*
  val (h, w) = (field.length, field.head.length)
  val initialBeams: Seq[Beam] =
    (0 until h).flatMap(y => Seq(Beam((y, 0), east), Beam((y, w - 1), west))) ++
      (0 until w).flatMap(x =>
        Seq(Beam((0, x), south), Beam((h - 1, x), north)),
      )
  initialBeams.map(energizedTileCount(_, field)).max

case class Beam(pos: Pos, dir: Pos):
  import Direction.*
  def step(f: Field): Seq[Beam] =
    val outgoingBeams =
      f(pos._1)(pos._2) match
        case '|' if isHorizontal => splitted
        case '-' if isVertical   => splitted
        case '/'                 => Seq(deflected((north, east), (south, west)))
        case '\\'                => Seq(deflected((north, west), (south, east)))
        case _                   => Seq(this)
    outgoingBeams.map(_.moved)

  private def moved = Beam((pos._1 + dir._1, pos._2 + dir._2), dir)

  private def isHorizontal = dir._1 == 0

  private def splitted: Seq[Beam] =
    if isVertical then Seq(turned(east), turned(west))
    else Seq(turned(north), turned(south))

  private def isVertical = dir._2 == 0

  private def turned(d: Pos) = Beam(pos, d)

  private def deflected(a: (Pos, Pos), b: (Pos, Pos)): Beam =
    val deflectedDirection =
      if dir == a._1 then a._2
      else if dir == a._2 then a._1
      else if dir == b._1 then b._2
      else b._1
    Beam(pos, deflectedDirection)

@main
def main(): Unit =
  val mirrors =
    Using(Source.fromFile("input.txt"))(src =>
      src.getLines().map(_.toArray).toArray,
    ).get
  val partOne = energizedTileCount(Beam((0, 0), Direction.east), mirrors)
  val partTwo = maxEnergizedTileCount(mirrors)
  println(s"Part one: $partOne, Part two: $partTwo")
