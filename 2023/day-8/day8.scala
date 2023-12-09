
import scala.io.Source
import scala.annotation.tailrec

type Moves = Vector[Char]
type DesertMap = Map[String, (String, String)]

def parseInput(string: String): (Moves, DesertMap) =
  val splitted = string.split("\n\n")
  val moves = raw"[LR]".r.findAllIn(splitted.head).map(_.head).toVector
  val desertMap = splitted.tail.head.split('\n').map(line =>
    val matches = raw"[A-Z]+".r.findAllIn(line)
    (matches.next, (matches.next, matches.next))
  ).toMap
  (moves, desertMap)

def lcm(factors: Seq[BigInt]): BigInt =
  @tailrec
  def gcd(a: BigInt, b: BigInt): BigInt = if (b == 0) a.abs else gcd(b, a % b)
  def lcm_(a: BigInt, b: BigInt) = (a * b).abs / gcd(a, b)
  factors.fold(1:BigInt)(lcm_)

def countMoves(initials: List[String], moves: Moves, desertMap: DesertMap): BigInt =
  // the idea is to compute howl ong it takes for each of the paths to complete, then we do
  // a "kleinstes gemeinsames Vielfaches", cycleNums stores the the number of moves for each path
  @tailrec
  def countPaths(currents: List[String], numMoves: Int, cycleNums: List[Int]): List[Int] =
    val nexts = currents.map(current =>
        moves(numMoves % moves.length) match
          case 'L' => desertMap(current)._1
          case 'R' => desertMap(current)._2
    )
    val (terminatedPaths, remainingPaths) = nexts.partition(_.endsWith("Z"))
    val finishedCycleNums = cycleNums ++ terminatedPaths.indices.map( _ => numMoves + 1)
    if remainingPaths.isEmpty then finishedCycleNums else countPaths(remainingPaths, numMoves + 1, finishedCycleNums)
  lcm(countPaths(initials, 0, List()).map(BigInt.apply))

@main
def main(): Unit =
  val input = Source.fromFile("input.txt").mkString
  val (moves, desertMap) = parseInput(input)
  val partOne = countMoves(List("AAA"), moves, desertMap)
  val partTwo = countMoves(desertMap.keys.filter(_.endsWith("A")).toList, moves, desertMap)
  println(s"Part one: $partOne, Part two: $partTwo")
