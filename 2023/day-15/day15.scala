
import scala.io.Source
import scala.util.Using
import scala.collection.mutable

type Boxes = Array[mutable.LinkedHashMap[String, Int]]

def parseInput(lines: Iterator[String]): Vector[String] = lines.flatMap(_.split(',').filter(_.nonEmpty)).toVector

def processChar(cv: Int, char: Char): Int = ((cv + char.toInt) * 17) % 256

def processString(str: String): Int = str.foldLeft(0)(processChar)

def processLensInstruction(boxes: Boxes)(str: String): Unit =
  if str.endsWith("-") then boxes(processString(str.init)).remove(str.init)
  else
    val labelAndFocalLength = str split "="
    val (label, focalLength) = (labelAndFocalLength(0),labelAndFocalLength(1).toInt)
    boxes(processString(label)).update(label, focalLength)

def focusingPower(boxes: Boxes): Int =
  (for
    (box, boxNum) <- boxes.zipWithIndex
    (lens, lensNum) <- box.zipWithIndex
  yield (boxNum + 1) * (lensNum + 1) * lens._2).sum

@main
def main(): Unit =
  val initSeq = Using(Source.fromFile("input.txt"))(src => parseInput(src.getLines())).get
  val partOne = (initSeq map processString).sum
  val boxes: Boxes = (0 until 256).map(_ => mutable.LinkedHashMap()).toArray
  initSeq foreach processLensInstruction(boxes)
  val partTwo = focusingPower(boxes)
  println(s"Part one: $partOne, Part two: $partTwo")
