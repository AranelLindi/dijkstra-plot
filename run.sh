#!/bin/bash

# TODO: Python script was commented to not destroy current plot png! Undo after finishing script!

read -p "Insert GraphML-file path: " path

if [ -z "$path" ]; then
	path="Graph.xml" # Important: no spaces between expression are allowed! (e.g. path = "Graph.xml")!
fi

echo $path # $ is necessary if value stored in variable is needed, otherwise variable itself will be used


echo "Dijkstra Plot is going to be executed..."

# Execute dijkstra-plot
./target/debug/dijkstra-plot -input=$path -output="Graph.dat" -start= -dest=

if [ $? -eq 1 ]; then # $? is a shell variable that contains exit code of last executed command (here: application)
  echo "Error in rust application!"
  exit 1 # End script with error code 1
fi

echo ""

# Check if Graph.dat was created in root directory
if [ -f "Graph.dat" ]; then
	echo "Graph.dat was created!"
else
	echo "Graph.dat was not created!"
fi

# Check if matlibplot library is installed
# shellcheck disable=SC2046
if [ $(pip show matplotlib >/dev/null 2>&1; echo $?) -eq 0 ]; then
    # Run python script
    python3 plot.py -input=$path -output="Graph2.png"
    echo "Plot is saved in Graph.png"
  else
    echo "Python script couldn't be executed because matlibplot library isn't installed on the system!"
fi

echo "Finished!"