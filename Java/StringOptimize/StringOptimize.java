class StringOptimize {
    public String StringVariable(String some) {
        String result = "";
        for (int i = 0; i < 10; i += 1) {
            result += some;
        }
        return result;
    }

    public String StringLiteral() {
        String result = "";
        for (int i = 0; i < 10; i += 1) {
            result += "some";
        }
        return result;
    }

    public String StringLocalVariable() {
        String result = "";
        String some = "some";
        for (int i = 0; i < 10; i += 1) {
            result += some;
        }
        return result;
    }
}
