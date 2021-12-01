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


windows : List a -> List $ Vect 3 a
windows xs = case xs of
  x :: y :: z :: rest => (x :: y :: z :: Nil) :: windows (y :: z :: rest)
  _ => Nil

countWindowedIncreases : List Int -> Int
countWindowedIncreases xs = snd $ foldl fun (Nothing, 0) (windows xs) where
  fun : (Maybe $ Vect 3 Int, Int) -> Vect 3 Int -> (Maybe $ Vect 3 Int, Int)
  fun (Just ol, count) nl = (Just nl, if (sum nl) > (sum ol) then count + 1 else count)
  fun (Nothing, count) nl = (Just nl, count)

export
partial solve : String -> String
solve str = show (countIncreases $ input, countWindowedIncreases input)
  where input : List Int
        input = parseInput str
