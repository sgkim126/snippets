import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.successful(10)
val f2 = Future.failed(new Exception("a"))
val f3 = Future.successful(100)

var x = 0
val f4 = Future.reduce(Seq(f1, f2, f3)) {
  (a, b) => {
    x += b
    a + b
  }
}

f4.onComplete(println) // Failure(java.lang.Exception: a)
println(x) // 0

val f5 = Future.successful(1000)
var y = 0
val f6 = Future.reduce(Seq(f5, f3, f1)) {
  (a, b) => {
    y += b
    throw new Exception("b")
  }
}
f6.onComplete(println) // Failure(java.lang.Exception: b)
println(y) // 1000
