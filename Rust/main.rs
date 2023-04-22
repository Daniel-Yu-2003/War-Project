#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Below is the function stub for deal. Add as many helper functions
    as you like, but the deal function should not be modified. Just
    fill it in.
    
    Test your code by running 'cargo test' from the war_rs directory.
*/

fn deal(shuf: &[u8; 52]) -> [u8; 52] // main deal function
{
    let pile = flipValue(shuf, 1, 14); // flip 1 to 14 to make 1 largest value
    let pile1 = dealCards(pile, 1); // deal two card piles
    let pile2 = dealCards(pile, 0);
    let winpile = flipValue(&(game(pile1, pile2)), 14, 1); // pile after game and 14 switched back to 1
    println!("{:?}", winpile);
    winpile
}

fn flipValue(pile: &[u8; 52], oldVal: u8, newVal: u8) -> [u8; 52] // flip values in array
{
    let mut newpile = pile.clone();
    for i in 0..newpile.len(){ // go through array and check which values to switch
        if newpile[i] == oldVal{ 
            newpile[i] = newVal;
        }
    }
    newpile
}

fn dealCards(pile: [u8; 52], check: u8) -> [u8; 26] // deal cards to a pile
{   
    let mut newpile = [0; 26];
    let mut count = 0;

    for i in (0..52).rev(){ // go through main pile in reverse
        if i % 2 == check{ // check which pile to add to
            newpile[count] = pile[i as usize];
            count = count + 1;
        }
    }
    newpile
}

fn game(pile1: [u8; 26], pile2: [u8; 26]) -> [u8; 52] // main game pile
{   
    let mut vec1 = pile1.to_vec(); // get piles as vector
    let mut vec2 = pile2.to_vec();

    loop{
        //println!("{:?}", vec1);
        let mut top1 = vec1[0];
        let mut top2 = vec2[0];
        let mut rest = vec![top1, top2]; // rest of pile or warchest
        rest.sort(); // sort and reverse the extra cards
        rest.reverse();
        vec1.remove(0); // remove first value from vector
        vec2.remove(0);
        if top1 == top2{ // check for war
            loop{
                if vec1.is_empty() || vec2.is_empty(){ // check for empty pile
                    return emptyTest(&mut vec1, &mut vec2, rest); // get the winning pile and return it
                }
                rest.extend([vec1[0], vec2[0]]); // add to rest of pile
                rest.sort(); 
                rest.reverse();
                
                vec1.remove(0); 
                vec2.remove(0);
                if vec1.is_empty() || vec2.is_empty(){
                    return emptyTest(&mut vec1, &mut vec2, rest);
                }
                rest.extend([vec1[0], vec2[0]]);
                rest.sort();
                rest.reverse();
                
                top1 = vec1[0];
                top2 = vec2[0];
                vec1.remove(0);
                vec2.remove(0);
                if top1 != top2{ //check for no war
                    if top1 > top2{  // check larger pile
                        vec1.extend(rest.clone()); // add to larger pile
                    }
                    else{
                        vec2.extend(rest.clone());
                    }
                    break;
                }
            }
        }
        else if top1 > top2{ // add rest of cards to end of first vector when top has larger value
            vec1.extend(rest.clone());
        }
        else{
            vec2.extend(rest.clone());
        }
        if vec1.is_empty() || vec2.is_empty(){ // check when any pile is empty
            return emptyTest(&mut vec1, &mut vec2, rest); // get the winning pile and return it
        }
    }
}

fn addInArr(vec: Vec<u8>) -> [u8; 52] // change vector to array
{
    let mut arr = [0; 52];
    for i in 0..52{
        arr[i] = vec[i];
    }
    arr
}

fn emptyTest(vec1: &mut Vec<u8>, vec2: &mut Vec<u8>, rest: Vec<u8>) -> [u8; 52] // check which pile is empty
{
    if vec1.is_empty() && vec2.is_empty(){ // both piles empty return rest pile
        return addInArr(rest.clone());
    }
    else if vec1.is_empty(){ // pile1 empty return pile2
        vec2.extend(rest);
        return addInArr(vec2.to_vec());
    }
    else{
        vec1.extend(rest);
        return addInArr(vec1.to_vec());
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

