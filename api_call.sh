#!/bin/bash

# Default loop 
loop_n=10

# Check if loop count_n is provided as argument
if [ $# -eq 1 ]; then
    loop_n=$1
fi

# Counter for the number of loops
count=0

while [ $count -lt $loop_n ]; do
    # Increment the counter
    ((count++))

    # Generate a random number between 0 and 1
    random_number=$(( RANDOM % 2 ))

    if [ $random_number -eq 0 ]; then
        # Call curl on localhost:8080
        curl localhost:8080
    else
        # Call curl on localhost:8080/health
        curl localhost:8080/health
    fi

    # Sleep for a random amount of time between 1 and 5 seconds
    sleep $(( RANDOM % 5 + 1 ))
done

