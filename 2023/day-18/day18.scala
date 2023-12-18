import scala.io.Source
import scala.util.Using

type Pos         = (Long, Long)
type Dir         = Pos
type Instruction = (Dir, Long, String)

val directions: Map[String, Pos] =
  Map("R" -> (1, 0), "L" -> (-1, 0), "U" -> (0, -1), "D" -> (0, 1))

extension (p: Pos)
  def x: Long             = p._1
  def y: Long             = p._2
  def +(q: Pos): Pos      = (p.x + q.x, p.y + q.y)
  def *(alpha: Long): Pos = (p.x * alpha, p.y * alpha)

def parseInput(lines: Iterator[String]): List[Instruction] =
  val matcher = raw"([LRUD])\s+(\d+)\s+\(#([0-9a-z]+)\)".r
  lines
    .toList
    .flatMap(line => matcher.findFirstMatchIn(line))
    .map(m => (directions(m.group(1)), m.group(2).toInt, m.group(3)))

def processInstructions(instructions: List[Instruction]): Long =
  val (_, perimeter, area) =
    instructions.foldLeft(((0L, 0L), 0L, 0L))((state, ins) =>
      val diff      = ins._1 * ins._2
      val vertexPos = state._1 + diff
      (vertexPos, state._2 + ins._2, state._3 + (vertexPos.x * diff.y)),
    )
  perimeter / 2 + area + 1

def transcode(instruction: Instruction): Instruction =
  val transcodeDirection = Map('0' -> "R", '1' -> "D", '2' -> "L", '3' -> "U")
  (
    directions(transcodeDirection(instruction._3.last)),
    BigInt(instruction._3.init, 16).intValue,
    instruction._3,
  )

@main
def main(): Unit =
  val instructions =
    Using(Source.fromFile("input.txt"))(src => parseInput(src.getLines())).get
  val partOne = processInstructions(instructions)
  val partTwo = processInstructions(instructions map transcode)
  println(s"Part one: $partOne, Part two: $partTwo")
