Connection conn = null;
Statement stmt = null;

try {
    conn = new Connection(connection);
    stmt = conn.createStatement();
    ResultSet result = stmt.executeQuery(query);
    // do something with result
} finally {
    stmt.close();
    conn.close();
}
