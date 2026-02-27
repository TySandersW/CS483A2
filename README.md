CS 483 Assignment 2 | Ty Sanders | 2.26.26
My solution to the problem is to iterate through every character in a string up until the halfway point, then compare each letter to its counterpart 
in the string. For instance, comparing the first character to the last, second to second to last, etc. This was done by prompting the user to input 
a string. I then iterated through the characters in the input by making the characters into a vector, something I learned off rust-lang.org, and 
collected the characters from the input. Then I made a Boolean value that shows whether the word is a palindrome or not. It is set to true, and if 
at any point while iterating the characters don’t match, it makes the value false and breaks the loop. It then returns whether or not the word is 
a palindrome or not. I also added some quality of life things, such as the function looping until the user types “q” to end the program.  
