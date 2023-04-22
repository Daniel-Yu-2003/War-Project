import Enum
defmodule War do
  @moduledoc """
    Documentation for `War`.
  """

  @doc """
    Function stub for deal/1 is given below. Feel free to add
    as many additional helper functions as you want.

    The tests for the deal function can be found in test/war_test.exs.
    You can add your five test cases to this file.

    Run the tester by executing 'mix test' from the war directory
    (the one containing mix.exs)
  """

  def deal(shuf) do # creates the 2 properly shuffed piles and plays the war game
		pile = Enum.map(shuf, fn x -> if x == 1, do: 14, else: x end) # turns the 1s into 14 to make it largest and reverse it at the end
		pile1 = Enum.drop_every(pile,2) # create the two piles
		pile1 = Enum.reverse(pile1)
		pile2 = Enum.drop_every([0 | pile], 2)
		pile2 = Enum.reverse(pile2)
		pile = war(pile1, pile2)
		Enum.map(pile, fn x -> if x == 14, do: 1, else: x end)
  end

	def war([], pile2) do pile2 end #cases where a pile is empty return the other pile
	def war(pile1, []) do pile1 end
  def war(pile1, pile2) do # the main war game
		top1 = hd pile1
		top2 = hd pile2
		extra = [top1, top2]
		if (top1 == top2) do # check for war and do other war function which stores the extra cards
			war((tl pile1), (tl pile2), extra)
		else

			if (top1 > top2) do # checks which card is larger, adds the cards to the winner, and continues the game
				pile1 = addToBottom(pile1, extra)
				war((tl pile1), (tl pile2))
			else
				pile2 = addToBottom(pile2, extra)
				war((tl pile1), (tl pile2))
			end
		end
  end
	def war([], [], extra) do addToBottom([],extra) end # case for war and both players run out of cards
	def war([], pile2, extra) do addToBottom(pile2, extra) end # cases where a player runes out of cards during war
	def war(pile1, [], extra) do addToBottom(pile1, extra) end
  def war(pile1, pile2, extra) do #function for wars

		extra = extra ++ [(hd pile1), (hd pile2)]
		pile1 = tl pile1
		pile2 = tl pile2

		if(empty?(pile1) or empty?(pile2)) do #check if any pile is empty
			war(pile1,pile2,extra)
		else
			extra = extra ++ [(hd pile1), (hd pile2)]
			if ((hd pile1) == (hd pile2)) do # check for war again and call the function again
				war((tl pile1), (tl pile2), extra)
			else
				if ((hd pile1) > (hd pile2)) do # check which card is larger and adds the extra cards to winners pile
					pile1 = addToBottom(pile1, extra)
					war((tl pile1), (tl pile2))
				else
					pile2 = addToBottom(pile2, extra)
					war((tl pile1), (tl pile2))
				end
			end
	end
end

  def addToBottom(pile, extra) do # sort the extra cards and add them to bottom of pile
		extra = Enum.sort(extra, :desc)
		addToBottomHelper(pile, extra)
  end

	def addToBottomHelper(pile,[]) do pile end # no more extra cards to add
	def addToBottomHelper(pile, extra) do # adds the card to the pile
		pile = pile ++ [(hd extra)]
		addToBottomHelper(pile, (tl extra))
	end
end
