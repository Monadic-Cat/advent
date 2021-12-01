module Day1
import Data.String
import Data.Maybe
import Data.Vect

-- What is Idris's version of collect::<Option<Vec<T>>()?
-- Well, it's fine. I can unwrap, since I know the input
-- is well formed.
partial unwrap : Maybe a -> a
unwrap (Just x) = x
partial parseInput : String -> List Int
parseInput str = let parse = \l => unwrap $ parsePositive l
           in map parse $ lines str

countIncreases : List Int -> Int
countIncreases xs = snd $ foldl fun (Nothing, 0) xs where
  fun : (Maybe Int, Int) -> Int -> (Maybe Int, Int)
  fun (last, count) x = (Just x, if fromMaybe False $ map (x >) last then count + 1 else count)

-- sum : Vect 3 Int -> Int
-- sum [x, y, z] = x + y + z

-- idris2 claims this isn't covering, but I can't see how
partial countWindowedIncreases : List Int -> Int
countWindowedIncreases xs = snd $ foldl fun ([Nothing, Nothing, Nothing], 0) xs where
  fun : (Vect 3 $ Maybe Int, Int) -> Int -> (Vect 3 $ Maybe Int, Int)
  fun ([Just x, Just y, Just z], count) c = (map Just nl, if (sum nl) > (sum ol) then count + 1 else count)
      where nl : Vect 3 Int
            nl = [y, z, c]
            ol : Vect 3 Int
            ol = [x, y, z]
  fun ([x, y, z], count) c = ([y, z, Just c], count)

export
partial solve : String -> String
solve str = show (countIncreases $ input, countWindowedIncreases input)
  where input : List Int
        input = parseInput str
