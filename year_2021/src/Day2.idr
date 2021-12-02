module Day2
import Data.String

import Util

data Command = Forward Int | Down Int | Up Int

raiseMaybe : List (Maybe a) -> Maybe (List a)
raiseMaybe lst = foldl fun (Just Nil) lst where
  fun : Maybe (List a) -> Maybe a -> Maybe (List a)
  fun (Just lst) (Just x) = Just (lst ++ [x])
  fun Nothing _ = Nothing
  fun _ Nothing = Nothing

parseCommands : String -> Maybe $ List Command
parseCommands input = raiseMaybe $ map matchCommand $ map words $ lines input where
  matchCommand : List String -> Maybe Command
  matchCommand [word, amt] = parsePositive {a = Int} amt >>=
    (\amt => case word of
      "forward" => Just $ Forward amt
      "down" => Just $ Down amt
      "up" => Just $ Up amt
      _ => Nothing)
  matchCommand _ = Nothing

record Displacement where
  constructor MkDisplacement
  horizontal, depth: Int

Show Displacement where
  show dsp = "horizontal: " ++ show (horizontal dsp)
             ++ ", depth: " ++ show (depth dsp)

calculateDisplacement : List Command -> Displacement
calculateDisplacement lst = foldl combine (MkDisplacement 0 0) lst where
  combine : Displacement -> Command -> Displacement
  combine a c = case c of
    Forward x => record { horizontal $= (+ x) } a
    Down x => record { depth $= (+ x) } a
    Up x => record { depth $= (\d => d - x) } a

calculateDisplacement' : List Command -> Displacement
calculateDisplacement' lst = fst $ foldl combine (MkDisplacement 0 0, 0) lst where
  combine : (Displacement, Int) -> Command -> (Displacement, Int)
  combine (dsp, aim) cmd = case cmd of
    Forward x => (record { horizontal $= (+ x), depth $= (+ aim * x) } dsp, aim)
    Down x => (dsp, aim + x)
    Up x => (dsp, aim - x)

prod : (List Command -> Displacement) -> List Command -> Int
prod cd cmds = let r = cd cmds
                   a = horizontal r
                   b = depth r
               in a * b

export
partial solve : String -> String
solve input = let cmds = unwrap $ parseCommands input
              in show (prod calculateDisplacement cmds, prod calculateDisplacement' cmds)
