def executeSql(connection: String)(block: Statement => Unit) {
  val conn = Connection(connection)
  val stmt = conn.createStatement()
  try {
    block(stmt)
  } finally {
    stmt.close()
    conn.close()
  }
}

val connection: String = /* ... */
executeSql(connection) { statement =>
  val query: String = /* ... */
  statement.executeQuery(query)
  /* ... */
}
