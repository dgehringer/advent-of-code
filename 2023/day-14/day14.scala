
import scala.io.Source
import scala.util.Using
import scala.collection.mutable

type Dish = Array[Array[Char]]

def parseInput(input: Iterator[String]): Dish = input.map(line => line.toArray).toArray

def rotateDish(dish: Dish): Dish = dish.transpose.map(_.reverse)

def tilt(dish: Dish): (Dish, BigInt) =
  var load = 0L
  val (h, w) = (dish.length, dish.head.length)
  val hashBuilder = StringBuilder("0" * (w * h))
  for x <- 0 until w do {
    var maxY = 0
    for y <- 0 until h do {
      dish(y)(x) match
        case 'O' =>
          if y > maxY then
            dish(maxY)(x) = 'O'
            dish(y)(x) = '.'
          hashBuilder(maxY * w + x) = '1'
          maxY += 1
        case '#' => maxY = y + 1
        case _ => ()
    }
  }
  (dish, BigInt(hashBuilder.toString(), 2))

def load(dish: Dish): Int =
  val h = dish.length
  dish.zipWithIndex.map((line, y) => (h -y) * line.count(_ == 'O')).sum

def cycle(dish: Dish): (Dish, BigInt) =
  val (tilted, hash) = (0 until 3).foldLeft(tilt(dish))((state, _) => tilt(rotateDish(state._1)))
  (rotateDish(tilted), hash)

def simulation(dish: Dish, n: Int): Long =
  var cache = mutable.HashMap[BigInt, Int]()
  var iterations = 0
  var state = cycle(dish)  // (dish, hash)
  // simulate until we find the same constellation again, which we identify by the BigInt hash
  while !cache.contains(state._2) do {
    cache.update(state._2, iterations)
    state = cycle(state._1)
    iterations += 1
  }
  val remainingIterations = (n - iterations) % (iterations - cache(state._2))
  // remaining loop cycles
  val (finalDish, _) = (0 until remainingIterations - 1).foldLeft(state)((s, _) => cycle(s._1))
  load(finalDish)

@main
def main(): Unit =
  val dish = Using(Source.fromFile("input.txt"))(src => parseInput(src.getLines())).get
  val partOne = load(tilt(dish.clone())._1)
  val partTwo = simulation(dish, 1000000000)
  println(s"Part one: $partOne, Part two: $partTwo")
