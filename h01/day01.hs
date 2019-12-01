module Main where

main :: IO()
main = do
         content <- readFile "input/day01"
         let numbers = readInt . words $ content
         let fule_sum = fuleReq numbers
         print $ fule_sum
         let better_fule_sum = betterFuleReq numbers
         print $ better_fule_sum

readInt :: [String] -> [Int]
readInt = map read

fuleReq :: [Int] -> Int 
fuleReq [] = 0
fuleReq (x:xs) = (div x 3) - 2 + fuleReq xs

betterFuleReq :: [Int] -> Int 
betterFuleReq [] = 0
betterFuleReq (x:xs) = fuleFuleReq x + betterFuleReq xs
                        where fuleFuleReq a | a > 0      = weight + fuleFuleReq weight
                                            | otherwise  = 0
                                            where weight = fuleReq [a]

