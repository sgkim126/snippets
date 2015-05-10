case class Address(address: String, port: Int)

val address: Address = Address("localhost", 80) // address : ("localhost", 80)
val Address(hostname, port) = address // hostname : "localhost, port : 80
// or
val hostname = address.hostname
val port = address.port

case class Date(month: String, day: Int)

val date: Date = Date("February", 21) // date : ("February", 21)
val Date(month, day) = date // month : "February, day : 21

val x: AnyRef = date
x match {
  case Address(h, p) => s"$h:$p"
  case Date(m, d) => s"$m $d"
  case _ => s"$x"
}

