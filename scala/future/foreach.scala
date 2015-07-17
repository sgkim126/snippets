import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.successful(Seq (1, 2, 3))

f1 foreach println // List(1, 2, 3)
