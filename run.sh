#!/bin/bash

# TODO: Python script was commented to not destroy current plot png! Undo after finishing script!

read -p "Insert GraphML-file path: " path

if [ -z "$path" ]; then
	path="Graph.xml" # Important: no spaces between expression are allowed! (e.g. path = "Graph.xml")!
fi

echo $path # $ is necessary if value stored in variable is needed, otherwise variable itself will be used


echo "Dijkstra Plot is going to be executed..."

# Execute dijkstra-plot
./target/debug/dijkstra-plot

echo ""

# Check if Graph.dat was created in root directory
if [ -f "Graph.dat" ]; then
	echo "Graph.dat was created!"
else
	echo "Graph.dat was not created!"
fi

# Run python script
#python3 plot.py

echo "Plot is saved in Graph.png"