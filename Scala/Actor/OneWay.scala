import akka.actor.{ ActorRef, ActorSystem, Props, Inbox }
import scala.concurrent.duration._

case class WhoToGreet(who: String)
case class Greet

object OneWay extends App {
  val system = ActorSystem("actorexample")

  val greeter = system.actorOf(Props[Greeter], "greeter")

  greeter.tell(WhoToGreet("akka"), ActorRef.noSender)
  greeter.tell(Greet, ActorRef.noSender)
}
