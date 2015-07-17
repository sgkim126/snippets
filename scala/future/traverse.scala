import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

var x1 = 0
val f1 = Future.traverse (Seq("abc", "d efg", "h")) {
  y => {
    x1 += 1
    if (y.length == 5) {
      Future.failed(new Exception())
    } else {
      Future.successful(y.length)
    }
  }
}

f1.onComplete(println) // Failure(java.lang.Exception)
println(x1) // 3

var x2 = 0
val f2 = Future.traverse (Seq("abc", "d efg", "h")) {
  y => {
    x2 += 1
    if (y.length == 5) {
      throw new Exception
    } else {
      Future.successful(y.length)
    }
  }
}

f2.onComplete(println) // Failure(java.lang.Exception)
println(x2) // 3
