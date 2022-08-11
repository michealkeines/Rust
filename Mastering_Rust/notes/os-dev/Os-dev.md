when a rust binary is run, the kernal starts a C libenv env and that calls the main method in the rust binary

for free binary we dont need that c runtime


when we normally run cargo test, the test framework will call a custom main function that it added to the start function

in our case, we make it to call our custom test main method that we write within the start method



