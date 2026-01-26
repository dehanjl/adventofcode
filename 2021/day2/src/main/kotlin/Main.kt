import java.io.File

typealias Command = Pair<String, Int>

fun part1(input: List<Command>): Int {
    var x = 0
    var depth = 0

    input.forEach { (direction, amount) ->
        when (direction) {
            "forward" -> x += amount
            "down" -> depth += amount
            "up" -> depth -= amount
        }
    }

    return x * depth
}

fun part2(input: List<Command>): Int {
    var x = 0
    var depth = 0
    var aim = 0

    input.forEach { (direction, amount) ->
        when (direction) {
            "forward" -> {
                x += amount
                depth += aim * amount
            }
            "down" -> aim += amount
            "up" -> aim -= amount
        }
    }

    return x * depth
}


fun main() {
    val input = File("input.txt").readLines().map {
        val (direction, amount) = it.split(" ")
        Command(direction, amount.toInt())
    }

    println("Part 1: ${part1(input)}")
    println("Part 2: ${part2(input)}")
}