#!/bin/bash

# Execute dijkstra-plot
./target/debug/dijkstra-plot

# Check if Graph.dat was created in root directory
if [ -f "Graph.dat" ]; then
	echo "Graph.dat was created!"
else
	echo "Graph.dat was not created!"
fi

# Run python script
python3 graph.py
