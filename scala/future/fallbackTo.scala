import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.failed(new Exception("a"))
val f2 = Future.failed(new Exception("b"))
val f3 = Future.successful(100)

val f4 = f1 fallbackTo f2
val f5 = f1 fallbackTo f3

f4.onComplete(println) // Failure(java.lang.Exception: a)
f5.onComplete(println) // Success(100)

val f6 = Future {
  Thread sleep 100000000
}

val f7 = f3 fallbackTo f6
f7.onComplete(println) // print Success(100) immediately


class A {
  override def toString: String = "A"
}
class B extends A {
  override def toString: String = "B"
}
val f8: Future[A] = Future.successful(new A)
val f9: Future[B] = Future.successful(new B)
val f10: Future[A] = Future.failed(new Exception ("a"))
val f11: Future[B] = Future.failed(new Exception ("b"))

f8 fallbackTo f9 onComplete println
f9 fallbackTo f8 onComplete println

f10 fallbackTo f8 onComplete println
f11 fallbackTo f9 onComplete println
