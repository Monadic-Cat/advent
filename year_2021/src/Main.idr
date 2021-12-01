module Main
import Control.App
import Control.App.Console
import Control.App.FileIO
import System.File

import Day1

partial days : App Init ()
days = do
  handle (readFile "input/day_1")
    (\str => putStrLn $ "Day 1: " ++ Day1.solve str)
    (\err : IOError => putStrLn $ "Error: " ++ show err)

partial main : IO ()
main = run days
