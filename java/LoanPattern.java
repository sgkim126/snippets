interface QueryExecutor {
    public abstract execute(Statement stmt);
}

class SomeQueryExecutor {
    public execute(Statement stmt) {
        String query = /* ... */;
        stmt.executeQuery(query);
        /* ... */
    }
}


public void executeSql(String connection, QueryExecutor executor) {
    Connection conn = Connection(connection);
    Statement stmt = conn.createStatement();
    try {
        executor.execute(stmt);
    } finally {
        stmt.close();
        conn.close();
    }
}
