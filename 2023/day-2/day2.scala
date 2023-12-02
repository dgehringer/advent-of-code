
import scala.io.Source
import scala.util.matching.Regex
import scala.util.matching.Regex.given

type Draw = Map[String, Int]
case class Game(val id: Int, val draws: List[Draw])

def emptyDraw: Map[String, Int] = Map[String, Int]()

def parseGame(line: String): Game =
  val matches = raw"((\d+)\s+(red|green|blue)(,\s+)?);?".r.findAllMatchIn(line)
  val(draws, firstDraw) = matches.foldRight((List[Draw](), emptyDraw))(
    (m, state) =>
      val pair = (m.group(3), m.group(2).toInt)
      if m.group(0).endsWith(";") then (state._2 :: state._1, Map(pair))
      else (state._1, state._2  + pair)
  )
  val gameId = raw"Game\s+(\d+):".r.findFirstMatchIn(line).get.group(1).toInt
  Game(gameId, firstDraw :: draws)

def isGamePossible(constraint: Draw)(game: Game): Boolean =
  game.draws.forall(_.forall((color, amount) => amount <= constraint(color)))

def mergeDraws(a: Draw, b: Draw): Draw =
  a.foldLeft(b.withDefaultValue(0))((r, e) => r updated (e._1, e._2 max r(e._1)))

def power(a: Draw): Int = a.values.product

@main
def main(): Unit =
  val maxCubes = Map("red" -> 12, "green" -> 13, "blue" -> 14)
  val games = (Source.fromFile("input.txt").getLines map parseGame).toList
  val partOne = games.filter(isGamePossible(maxCubes)).map(_.id).sum
  val partTwo = games.map(_.draws.reduce(mergeDraws)).map(power).sum
  println(s"Part one: $partOne, Part two: $partTwo")