
import scala.io.Source

type Race = (Long, Long)
def distance(total: Long, charge: Long): Long = charge * (total - charge)

def parseInput(input: Iterator[String]): (List[Race], Race) =
  val integerRegex = raw"\d+".r
  val lines = input.take(2).toList
  val times = integerRegex.findAllIn(lines.head).toList
  val records = integerRegex.findAllIn(lines.tail.head).toList
  (times.map(_.toLong) zip records.map(_.toLong), (times.mkString.toLong, records.mkString.toLong))

/*
 the distance travelled as a function of starting time is a quadratic function.
 So for a given record time {R} we find R = charge * (total - charge). Therefore
 for a given {R} all numbers between the positive an negative root will beat the
 current record, because of monotonicity of a parabola.
 That gives (-total (+-) sqrt(total^2 - 4R))/(-2)
 */
@inline
def distanceRoots(race: Race): (Double, Double) =
  val sqrtTerm = math.sqrt(math.pow(race._1.toDouble, 2) - 4.0 * race._2)
  val (x1, x2) = ((-race._1.toDouble - sqrtTerm)/(-2.0), (-race._1.toDouble + sqrtTerm)/(-2.0))
  (x1 min x2, x1 max x2)

@inline
def closeToRoot(race: Race)(root: Double): Long =
  if root.isValidInt then // the root is an integer value, we have to look to the left and right too
    List(root.longValue - 1, root.longValue, root.longValue + 1).maxBy(distance(race._1, _))
  else if distance(race._1, root.floor.toLong) > race._2 then root.floor.toLong
  else root.ceil.toLong

def beatingRangeLength(race: Race): Long  =
  val roots = distanceRoots(race)
  val closeForRace = closeToRoot(race)
  val (start, last) = (closeForRace(roots._1), closeForRace(roots._2))
  last - start + 1

@main
def main(): Unit =
  val (races, bigRace) = parseInput(Source.fromFile("input.txt").getLines)
  val partOne = races.map(beatingRangeLength).product
  val partTwo = beatingRangeLength(bigRace)
  println(s"Part one: $partOne, Part two: $partTwo")
