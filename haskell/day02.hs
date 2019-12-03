module Main where

import Data.List.Split

main :: IO()
main = do
         content <- readFile "../input/day02"
         let program = readInt . splitOn "," $ content
         let program = replace 1 12 program
         let program = replace 2 2 program
         let program = evalProgram 0 program
         print $ head program

readInt :: [String] -> [Int]
readInt = map read

replace :: Int -> Int -> [Int] -> [Int]
replace 0 n (_:xs) = n : xs
replace a n (q:xs) = q : replace (a - 1) n xs

evalProgram :: Int -> [Int] -> [Int]
evalProgram at prog | op == 1  = evalProgram (at + 4) $ evalAdd a b r prog
                    | op == 2  = evalProgram (at + 4) $ evalMul a b r prog
                    | op == 99 = prog
                    where op = prog !! (at + 0)
                          a  = prog !! (at + 1)
                          b  = prog !! (at + 2)
                          r  = prog !! (at + 3)

evalAdd :: Int -> Int -> Int -> [Int] -> [Int]
evalAdd a b r prog = replace r q prog
                     where q = a + b

evalMul :: Int -> Int -> Int -> [Int] -> [Int]
evalMul a b r prog = replace r q prog
                     where q = a * b
