
import scala.annotation.tailrec
import scala.io.Source
import scala.util.Using

type Pos = (Int, Int)
type Path = List[Pos]
type Field = Array[String]

def parseInput(s: String): (Pos, Field) =
  val field = s
    .split("\n")
  val y = field.indexWhere(_.exists(_ == 'S'))
  val x = field(y).indexWhere(_ == 'S')
  ((y, x), field)

def add(a: Pos, b: Pos): Pos = (a._1 + b._1, a._2 + b._2)

case class Pipe(in: Pos, out: Pos):
  def nextOn(path: Path): Pos = if in == path.tail.head then out else in
  def isConnectedTo(pos: Pos): Boolean = in == pos || out == pos

def makePipe(pos: Pos, field: Field): Option[Pipe] =
  val addTo = add.curried(pos)
  field(pos._1)(pos._2) match
    case '|' => Some(Pipe(addTo((-1, 0)), addTo((1, 0))))
    case '-' => Some(Pipe(addTo((0, 1)), addTo((0, -1))))
    case 'L' => Some(Pipe(addTo((-1, 0)), addTo((0, 1))))
    case 'J' => Some(Pipe(addTo((-1, 0)), addTo((0, -1))))
    case '7' => Some(Pipe(addTo((0, -1)), addTo((1, 0))))
    case 'F' => Some(Pipe(addTo((0, 1)), addTo((1, 0))))
    case _ => None

def findLoop(start: Pos, field: Field): Path =
  val (h, w) = (field.length, field.head.length)
  def inBounds(pos: Pos): Boolean = pos._1 >= 0 && pos._1 < h && pos._2 >= 0 && pos._2 < w

  def growPath(path: Path): Option[Path] =
    val nextTile = makePipe(path.head, field).get nextOn path
    if nextTile == start then Some(nextTile :: path)
    else makePipe(nextTile, field) match
      case Some(pipe) => if inBounds(nextTile) && (pipe isConnectedTo path.head) then Some(nextTile :: path) else None
      case None => None

  @tailrec
  def searchForLoop(path: Path): Option[Path] =
    if path.head == start then Some(path)
    else growPath(path) match
      case Some(p) => if p.head == start then Some(p) else searchForLoop(p)
      case None => None

  val allowed = List((0, 1), (0, -1), (1, 0), (-1, 0))
  val paths = allowed.map(d => add(start, d)).filter(inBounds).map(List(_, start)).flatMap(searchForLoop)
  paths
    .flatMap(p1 => paths.map((p1, _)))
    .find(_ == _.reverse)
    .getOrElse((Nil, Nil))
    ._1.init

def maxDistance(path: Path): Int =
  val halfSize = path.length / 2
  if path.length % 2 == 0 then  halfSize else halfSize + 1

def countTilesInside(field: Field, loop: Path): Int =
  val (h, w) = (field.length, field.head.length)
  val loopLookup = loop.toSet
  var count = 0
  for y <- 0 until h do
    var inside = false
    for x <- 0 until w do
      if loopLookup contains (y, x) then
        if ("|JL" contains field(y)(x)) || field(y)(x) == 'S' then
          inside = !inside
      else if inside then
        count += 1
  count

@main
def main(): Unit =
  val (start, field) = Using(Source.fromFile("input.txt"))(s => parseInput(s.mkString)).get
  val loop = findLoop(start, field)
  val partOne = maxDistance(loop)
  val partTwo = countTilesInside(field, loop)
  print(s"Part one: $partOne, Part two: $partTwo")

