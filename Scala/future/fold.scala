import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.successful(10)
val f2 = Future.failed(new Exception("a"))
val f3 = Future.successful(100)

var x = 0
val f4 = Future.fold(Seq(f1, f2, f3))(0) {
  (a, b) => {
    x += b
    a + b
  }
}

f4.onComplete(println) // Failure(java.lang.Exception: a)
println(x) // 0
