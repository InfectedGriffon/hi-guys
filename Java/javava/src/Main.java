import java.util.function.Predicate;

public class Main {
    public static Predicate<Integer> is_even = n -> n % 2 == 0;

    public static void main(String[] args) {
        System.out.println("Hello world!");
        System.out.printf("is four even? %b", is_even.test(4));
    }
}
