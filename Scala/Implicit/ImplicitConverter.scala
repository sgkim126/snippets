case class Size(width: Int, height: Int)
case class Point(x: Int, y: Int)
case class Rectangle(point: Point, size: Size)

implicit def sizeToRectangle(size: Size): Rectangle = Rectangle(Point(0, 0), size)
val r: Rectangle = Size(20, 20)
