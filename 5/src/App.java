import java.io.IOException;
import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;


public class App {
    private static String input;
    private static String example;
    private static List<String> arr;

    private static BigInteger mapper(BigInteger value, String mapLine) {
        List<String> line = Arrays.asList(mapLine.split(" "));
        BigInteger dest = new BigInteger(line.get(0).trim());
        BigInteger source = new BigInteger(line.get(1).trim());
        BigInteger steps = new BigInteger(line.get(2).trim()).subtract(BigInteger.ONE);
        //System.out.println(line.get(2) + " steps: " + steps);
        // for(BigInteger j = BigInteger.valueOf(0L); j.compareTo(steps) == -1; j = j.add(BigInteger.ONE)) {
        //     ret.put(new BigInteger(line.get(1)).add(j), new BigInteger(line.get(0)).add(j));
        //     //System.out.println(j);
        // }
        if(value.compareTo(source) >= 0 && value.compareTo(source.add(steps)) <= 0) {
            //System.out.println(value + "->" + dest.add(value.subtract(source)));
            return dest.add(value.subtract(source));

        }
        return value;
    }

    public static BigInteger solveOne(List<BigInteger> seeds, List<String> arr) {
        //System.out.println(3127166940 < BigInteger.MAX_VALUE);
        List<BigInteger> values = seeds;

        for(int j = 0; j < values.size(); j++) {
            System.out.println("Seed: " + seeds.get(j) + " index: " + j);
            for(int l = 1; l <= 7; l++) {
                List<String> maps = Arrays.asList(arr.get(l).split("\\r\\n"));
                for(int i = 1; i < maps.size(); i++) {
                    BigInteger newVal = mapper(values.get(j), maps.get(i));
                    if(newVal != values.get(j)) {
                        values.set(j, newVal);
                        break;
                    }
                }
                //System.out.println("Map: " + l + " Currently: " + values.get(j) + "\n");
                //System.out.println("Seed: " + seeds.get(j) + " Currently: " + values.get(j));
            }
            System.out.println("----------------------------------");
        }

        System.out.println(seeds);

        BigInteger lowestLocation = values.get(0);

        for(int i = 0; i < values.size(); i++) {
            if(lowestLocation.compareTo(values.get(i)) == 1) {
                lowestLocation = values.get(i);
            }
        }

        return lowestLocation;
    }

    public static void main(String[] args) throws IOException {

        input = Files.readString(Paths.get("5/src/input.txt").toAbsolutePath());
        example = Files.readString(Paths.get("5/src/example.txt").toAbsolutePath());
        arr = Arrays.stream(input.split("(\\r\\n){2}"))
            .filter(s -> s.trim().length() >= 1)
            .collect(Collectors.toList());

        List<BigInteger> seeds = new ArrayList();
        Arrays.stream(arr.get(0).split(":")[1].split(" "))
            .filter(s -> s.trim().length() >= 1)
            .forEach(numStr -> seeds.add(new BigInteger(numStr.trim())));
        //System.out.println(seeds);
        System.out.println(solveOne(seeds, arr));
        }


}