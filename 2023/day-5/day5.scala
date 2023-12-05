
import scala.io.Source

type Range = (Long, Long)
type MappingRange = (Long, Long, Long)

def parseInput(string: String): (List[Long], List[Mapping]) =
  val integerRegex = raw"\d+".r
  val blocks = string.split("\n\n")
  val seeds = integerRegex.findAllIn(blocks.head).map(_.toLong).toList
  val maps =
    blocks.tail
      .filterNot(_.isEmpty)
      .map(block =>
        val lines = block.split('\n')
        val blockDescription = raw"(\w+)-to-(\w+)".r.findFirstMatchIn(lines.head).get
        val ranges = lines.tail.map(line =>
          val numbers = integerRegex.findAllIn(line).map(_.toLong).toList
          assert(numbers.length == 3)
          (numbers.head, numbers(1), numbers(2))
        )
        Mapping(blockDescription.group(1), blockDescription.group(2), ranges.sortBy(_._2))
      ).toList
  (seeds, maps)

@inline
def inBounds(start: Long, range: Long, value: Long): Boolean = value >= start && value < (start + range)

extension[T] (ls: List[T])
  def updatedIf(cond: Boolean)(el: T): List[T] =
    if cond then el :: ls else ls

case class Mapping(source: String, dest: String, ranges: Seq[MappingRange]):
  def mapSeed(v: Long): Long = ranges.find(r => inBounds(r._2, r._3, v)) match
    case Some(range) => v + range._1 - range._2
    case None => v

  private def applyMappingRange(m: MappingRange, v: Long): Long = m._1 + v - m._2

  def mapSeedRanges(seedRange: Range): List[Range] =
    val foldInit: (Long, Long, List[Range]) = (seedRange._1, seedRange._2, List[Range]())
    val (start: Long, _, ranges: List[Range]) =
      this.ranges.foldLeft(foldInit)((state, mr) =>
        val (seedStart, seedEnd, seedRanges) = state
        val intersectionStart = seedStart max mr._2
        val intersectionEnd = seedRange._2 min (mr._2 + mr._3 - 1)
        if intersectionStart <= intersectionEnd then
          val mappedRange = (applyMappingRange(mr, intersectionStart), applyMappingRange(mr, intersectionEnd))
          val newSeed = if intersectionEnd < seedEnd then intersectionEnd + 1 else Long.MaxValue
          val newRanges = (mappedRange :: seedRanges).updatedIf(seedStart < intersectionStart)(seedStart, intersectionStart - 1)
          (newSeed, seedEnd, newRanges)
        else state
      )
    ranges.updatedIf(start < seedRange._2)((start, seedRange._2))

@main
def main(): Unit =
  val inputData = Source.fromFile("input.txt").mkString
  val (seeds, maps) = parseInput(inputData)
  val partOne = seeds.map(seed => maps.foldLeft(seed)((s, m) => m mapSeed s)).min
  val seedRanges = seeds.grouped(2).map(l => (l.head, l.head - 1 + l.tail.head)).toList
  val partTwo =
    maps
      .foldLeft(seedRanges)(_ flatMap _.mapSeedRanges)
      .map(_._1)
      .min
  println(s"Part one: $partOne Part two: $partTwo")
