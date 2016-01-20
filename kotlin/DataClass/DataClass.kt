import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.ObjectInputStream
import java.io.ObjectOutputStream
import java.io.Serializable

data class DataClass(val a: Int, var b: String): Serializable

fun main(args: Array<String>) {
    val a = DataClass(1, "a")
    println("${a}")

    val filename = "./dataclass"
    val fos = FileOutputStream(filename)
    val oos = ObjectOutputStream(fos);
    oos.writeObject(a)
    oos.close()

    val fis = FileInputStream(filename)
    val ois = ObjectInputStream(fis)
    val b = ois.readObject() as DataClass
    ois.close()

    println("${b}")
}
