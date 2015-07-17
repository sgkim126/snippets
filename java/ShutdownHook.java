class ShutdownHook {
    static public void main(String[] args) throws Exception {
        Runtime.getRuntime().addShutdownHook(new Thread() {
            public void run() {
                System.out.println("shutdown");
            }
        });
        throw new Exception();
    }
}
