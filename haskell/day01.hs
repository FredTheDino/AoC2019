module Main where

main :: IO()
main = do
         content <- readFile "../input/day01"
         let numbers = readInt . words $ content
         let fuel_sum = fuelReq numbers
         print $ fuel_sum
         let better_fuel_sum = betterFuelReq numbers
         print $ better_fuel_sum

readInt :: [String] -> [Int]
readInt = map read

fuelReq :: [Int] -> Int 
fuelReq [] = 0
fuelReq (x:xs) = (div x 3) - 2 + fuelReq xs

betterFuelReq :: [Int] -> Int 
betterFuelReq [] = 0
betterFuelReq (x:xs) = fuelFuelReq x + betterFuelReq xs
            where fuelFuelReq a | weight > 0 = weight + fuelFuelReq weight
                                | otherwise  = 0
                                where weight = fuelReq [a]

