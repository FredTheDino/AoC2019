module Main where

import Data.List.Split

main :: IO()
main = do
         content <- readFile "../input/day02"
         let program = readInt $ splitOn "," content
         let program_a = replace 1 12 program
         let program_b = replace 2 2 program_a
         let program_c = evalProgram 0 program_b
         print $ head program_c

readInt :: [String] -> [Int]
readInt = map read

replace :: Int -> Int -> [Int] -> [Int]
replace n a xs = take n xs ++ [a] ++ drop (n + 1) xs

evalProgram :: Int -> [Int] -> [Int]
evalProgram at prog | op == 1   = evalProgram (at + 4) $ evalAdd a b r prog
                    | op == 2   = evalProgram (at + 4) $ evalMul a b r prog
                    | op == 99  = prog
                    | otherwise = undefined
                    where op = prog !! (at + 0)
                          a  = prog !! (prog !! (at + 1))
                          b  = prog !! (prog !! (at + 2))
                          r  = prog !! (at + 3)

evalAdd :: Int -> Int -> Int -> [Int] -> [Int]
evalAdd a b r prog = replace r q prog
                     where q = a + b

evalMul :: Int -> Int -> Int -> [Int] -> [Int]
evalMul a b r prog = replace r q prog
                     where q = a * b
