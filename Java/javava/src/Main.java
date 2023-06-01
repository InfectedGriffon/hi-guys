public class Main {
    public static void main(String[] args) {
        Greeter greeter = new GreeterFactory().build();
        World world = new WorldBuilder().build();
        greeter.greet(world.get());
    }
}
