module War (deal) where
import Data.List ( sort )

{--
Function stub(s) with type signatures for you to fill in are given below. 
Feel free to add as many additional helper functions as you want. 

The tests for these functions can be found in src/TestSuite.hs. 
You are encouraged to add your own tests in addition to those provided.

Run the tester by executing 'cabal test' from the war directory 
(the one containing war.cabal)
--}
    
deal shuf = do           
    let pile = flipVal (shuf, [], 1, 14) -- flip 1 and 14 so that 1 turns into largest value
        pile1 = reverse (dealCards (pile, 1, [])) -- deal cards and reverse the 2 piles
        pile2 = reverse (dealCards (pile, 0, []))
        winpile = game(pile1, pile2) -- call the main game function
        flipwinpile = flipVal (winpile, [], 14, 1) -- flip the 14 and 1 so that is back to normal
    flipwinpile

flipVal (shuf, new, oldVal, newVal) -- flip 1 to 14 so that 1 turns to biggest number
    |   null shuf = new -- return new list when old list is empty
    |   head shuf == oldVal = flipVal(tail shuf, new ++ [newVal], oldVal, newVal) -- when when its old value, switch with new value and call method again
    |   otherwise = flipVal(tail shuf, new ++ [head shuf], oldVal, newVal) -- add value to list and call method again

dealCards (pile, skip, new) -- deal cards to a pile
    |   null pile = new -- return new pile when old pile is empty
    |   skip == 1 = dealCards(tail pile, 0, new ++ [head pile]) -- when skip is 1 add top of list to new pile
    |   otherwise = dealCards(tail pile, 1, new) -- skip top of list

game ([], pile2) = pile2 -- pile1 is empty return pile2
game (pile1, []) = pile1 -- pile2 is empty return pile1
game (pile1, pile2) = do -- main game function
    let top1 = head pile1 
        top2 = head pile2
        temp = [top1, top2]
    if top1 == top2 then -- war happens
        war(tail pile1, tail pile2, temp) -- call war function
    else do
        if top1 > top2 then 
            game (tail (addToBottom(pile1, temp)), tail pile2) -- call game method again with new piles
        else
            game (tail pile1, tail (addToBottom(pile2, temp)))

war ([],[],temp) = addToBottom ([],temp) -- both piles empty
war (pile1, [], temp) = addToBottom(pile1, temp) 
war ([], pile2, temp) = addToBottom(pile2, temp)
war (pile1, pile2, temp) = do -- main war function
    emptyTest(tail pile1,tail pile2, temp ++ [head pile1, head pile2]) -- check if any tail pile is empty
        

emptyTest(pile1, pile2, temp) -- check for empty pile
    |   null pile1 || null pile2 = war(pile1, pile2, temp) -- empty pile call war function again
    |   otherwise = warTest(pile1, pile2, temp ++ [head pile1, head pile2]) -- not empty call wartest function

warTest(pile1, pile2, temp) -- during war test values
    | head pile1 == head pile2 = war(tail pile1, tail pile2, temp) 
    | head pile1 > head pile2 = game(tail (addToBottom(pile1, temp)), tail pile2)
    | otherwise = game(tail pile1, tail (addToBottom(pile2, temp)))

addToBottom (pile, temp) = do -- add warchest to bottom of a pile
    let sorttemp = reverse(sort temp) -- reverse and sort warchest
    addToBottomMain(pile, sorttemp)

addToBottomMain(pile, []) =  pile -- empty warchest return pile
addToBottomMain(pile, temp) = do -- add warchest to pile 1 by 1
    let new = pile ++ [head temp]
    addToBottomMain(new, tail temp)

               
    
